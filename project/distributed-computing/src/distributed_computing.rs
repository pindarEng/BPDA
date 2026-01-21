#![no_std]

// use core::task;

use core::task;

#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc::{derive_imports::*};



#[type_abi]
#[derive(TopEncode, TopDecode,ManagedVecItem,NestedEncode, NestedDecode, Debug, PartialEq)]
pub enum TaskStatus {
    Open,
    InVerification,
    Completed,
    Failed,
}

#[type_abi]
#[derive(TopEncode, TopDecode, ManagedVecItem, NestedEncode, NestedDecode, Debug, PartialEq)]
pub struct Task<M: ManagedTypeApi>{
    pub creator: ManagedAddress<M>,
    pub docker_image_uri: ManagedBuffer<M>,
    pub input_data_uri: ManagedBuffer<M>,
    pub reward_amount: BigUint<M>,
    pub max_workers: usize,
    pub submissions_count: usize, // the number of workers posting a submission, if this is less than the max_workers that means free space is available for others to participate
    pub status: TaskStatus,
}



//An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait DistributedComputing {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    //storages


    // #[view(getTaskIdCounter)]
    #[storage_mapper("task_id_counter")]
    fn task_id_counter(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("tasks")]
    fn tasks(&self, task_id: u64) -> SingleValueMapper<Task<Self::Api>>;

    #[storage_mapper("worker_submissions")]
    fn worker_submissions(&self, task_id: u64, worker: &ManagedAddress) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("task_worker_list")]
    fn task_worker_list(&self, task_id: u64) -> VecMapper<ManagedAddress>;

    #[storage_mapper("hash_frequency")]
    fn hash_frequency(&self, task_id: u64, hash: &ManagedBuffer) -> SingleValueMapper<usize>;


    // requester endpoints
    #[payable("EGLD")]
    #[endpoint(postTask)]
    fn post_task(&self, docker_image_uri: ManagedBuffer, input_data_uri: ManagedBuffer, max_workers: usize){
        let payment = self.call_value().egld();
        require!(
            *payment > 0 , "reward must be greater than 0"
        );
        require!(
            max_workers >= 1, "at least 3 workers for a consesus to be reached"
        );

        let task_id = self.task_id_counter().get();
        let task = Task {
            creator: self.blockchain().get_caller(),
            docker_image_uri,
            input_data_uri,
            reward_amount: payment.clone_value(),
            max_workers,
            submissions_count: 0,
            status: TaskStatus::Open,
        };

        self.tasks(task_id).set(&task);
        self.task_id_counter().update(|id| *id +=1)
    }


    // worker endpoint

    #[endpoint(submitResult)]
    fn submit_result(&self, task_id: u64, result_hash: ManagedBuffer){
        let mut task = self.tasks(task_id).get();
        let caller = self.blockchain().get_caller();

        require!(
            task.status == TaskStatus::Open, "task is not open!!!"
        );

        require!(
            self.worker_submissions(task_id, &caller).is_empty(), "worker alrdy submitted"
        );

        self.worker_submissions(task_id, &caller).set(&result_hash);
        self.task_worker_list(task_id).push(&caller);

        self.hash_frequency(task_id, &result_hash).update(|count| * count += 1);
        task.submissions_count += 1;

        if task.submissions_count == task.max_workers {
            task.status = TaskStatus::InVerification;
            self.finalize_task(task_id, &mut task);
        }

        self.tasks(task_id).set(&task);
    }


    // logic functions

    fn finalize_task(&self, task_id: u64, task: &mut Task<Self::Api>){
        let mut winning_hash = ManagedBuffer::new();
        let mut max_votes = 0usize;
        let majority_threshhold = (task.max_workers / 2) + 1;

        for worker in self.task_worker_list(task_id).iter(){
            let hash = self.worker_submissions(task_id, &worker).get();
            let votes = self.hash_frequency(task_id, &hash).get();

            if votes >= majority_threshhold {
                winning_hash = hash;
                max_votes = votes;
                break;
            }
        }

        if max_votes >= majority_threshhold {
            self.distribute_rewards(task_id, task, &winning_hash, max_votes);
            task.status = TaskStatus::Completed;
        } else { // NO consensus
            self.send().direct_egld(&task.creator, &task.reward_amount);
            task.status = TaskStatus::Failed;
        }

    }

    fn distribute_rewards(&self, task_id: u64, task: &Task<Self::Api>, winning_hash: &ManagedBuffer, winner_count: usize){
        let share = &task.reward_amount / (winner_count as u64);
        for worker in self.task_worker_list(task_id).iter(){
            let hash = self.worker_submissions(task_id, &worker).get();
            if &hash == winning_hash {
                self.send().direct_egld(&worker, &share);
            }
        }
    }


    #[view(getTask)]
    fn get_task(&self, task_id: u64) -> Task<Self::Api> {
        self.tasks(task_id).get()
    }

    #[view(getTaskStatus)]
    fn get_task_status(&self, task_id: u64) -> TaskStatus {
        let task = self.tasks(task_id).get();
        task.status
    }

}
