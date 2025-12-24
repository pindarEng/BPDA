import { useEffect, useState } from 'react';
import { Address, AbiRegistry, SmartContractController } from '@multiversx/sdk-core';
import { ProxyNetworkProvider, formatAmount } from 'lib';
import { contractAddress } from 'config';
import { Task, TaskStatus } from 'types/distributedComputing.types';
import { Button } from 'components/Button';
import { OutputContainer } from 'components/OutputContainer';
import jsonAbi from 'contracts/distributed-computing.abi.json';
import { useGetNetworkConfig } from 'lib';
import { faCopy, faTerminal, faPlay } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

export const TaskBoard = () => {
  const { network } = useGetNetworkConfig();
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(false);
  const [copiedId, setCopiedId] = useState<number | null>(null);
  const [processingId, setProcessingId] = useState<number | null>(null);

  const copyWorkerCommand = (task: Task) => {
    const command = `python3 worker.py --task-id ${task.id} --wallet ./wallet.pem --image "${task.docker_image_uri}" --input "${task.input_data_uri}"`;
    navigator.clipboard.writeText(command);
    setCopiedId(task.id);
    setTimeout(() => setCopiedId(null), 2000);
  };

  const handleParticipate = async (task: Task) => {
    setProcessingId(task.id);
    try {
      const response = await fetch('http://localhost:5005/process_task', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          taskId: task.id,
          image: task.docker_image_uri,
          inputData: task.input_data_uri
        }),
      });

      const data = await response.json();
      
      if (response.ok) {
        alert(`Task processed successfully! Tx: ${data.txHash}`);
        fetchTasks();
      } else {
        alert(`Error processing task: ${data.message || 'Unknown error'}`);
      }
    } catch (error) {
      console.error('Error calling worker server:', error);
      alert('Failed to connect to local worker server. Is it running on port 5005?');
    } finally {
      setProcessingId(null);
    }
  };

  const fetchTasks = async () => {
    if (!network.apiAddress) return;
    
    setLoading(true);
    try {
      const provider = new ProxyNetworkProvider(network.apiAddress);
      const abiRegistry = AbiRegistry.create(jsonAbi);
      
      const controller = new SmartContractController({
        chainID: network.chainId,
        networkProvider: provider,
        abi: abiRegistry
      });

  const fetchedTasks: Task[] = [];
      
      // Try to fetch first 10 tasks
      for (let i = 0; i < 10; i++) {
        try {
          const interaction = controller.createQuery({
            contract: new Address(contractAddress),
            function: 'getTask',
            arguments: [i]
          });
          
          const queryResponse = await controller.runQuery(interaction);
          const parsedResponse = controller.parseQueryResponse(queryResponse);
          
          // parsedResponse is an array of values. 
          // Since getTask returns a Task struct, parsedResponse[0] should be the Task object.
          
          if (parsedResponse && parsedResponse.length > 0) {
            const taskData = parsedResponse[0];

            const parseStatus = (raw: any): TaskStatus => {
              try {
                if (raw === undefined || raw === null) return TaskStatus.Open;
                
                // Handle SDK Enum object (has name property)
                if (typeof raw === 'object' && 'name' in raw) {
                   return raw.name as TaskStatus;
                }
                
                // Handle object with index/ordinal
                if (typeof raw === 'object' && 'index' in raw) {
                   const vals = Object.values(TaskStatus);
                   return (vals[raw.index] ?? TaskStatus.Open) as TaskStatus;
                }

                // if it's an object with toNumber (protobuf/bignumber), try to coerce
                const maybeNumber =
                  typeof raw === 'object' && raw !== null && ('toNumber' in raw || 'toFixed' in raw)
                    ? Number((raw as any).toNumber ? (raw as any).toNumber() : (raw as any).toFixed())
                    : Number(raw);

                if (!Number.isNaN(maybeNumber)) {
                  const vals = Object.values(TaskStatus);
                  return (vals[maybeNumber] ?? TaskStatus.Open) as TaskStatus;
                }

                const str = String(raw);
                const found = Object.values(TaskStatus).find((v) => v.toLowerCase() === str.toLowerCase());
                return (found ?? TaskStatus.Open) as TaskStatus;
              } catch (e) {
                return TaskStatus.Open;
              }
            };

            fetchedTasks.push({
              id: i,
              creator: taskData.creator?.toString ? taskData.creator.toString() : String(taskData.creator),
              docker_image_uri: taskData.docker_image_uri?.toString ? taskData.docker_image_uri.toString() : String(taskData.docker_image_uri),
              input_data_uri: taskData.input_data_uri?.toString ? taskData.input_data_uri.toString() : String(taskData.input_data_uri),
              reward_amount: taskData.reward_amount?.toString ? taskData.reward_amount.toString() : String(taskData.reward_amount),
              max_workers: Number(taskData.max_workers ?? 0),
              submissions_count: Number(taskData.submissions_count ?? 0),
              status: parseStatus(taskData.status)
            });
          }
        } catch (err) {
          // Ignore errors for non-existent tasks or if we reached the end
          // console.warn(`Failed to fetch task ${i}`, err);
        }
      }
      
      setTasks(fetchedTasks);
    } catch (error) {
      console.error("Error fetching tasks", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTasks();
  }, [network.apiAddress]);

  return (
    <div className="flex flex-col gap-4">
      <div className="flex justify-between items-center">
        <h3 className="text-xl font-bold">Available Tasks</h3>
        <Button onClick={fetchTasks} disabled={loading}>
          {loading ? 'Loading...' : 'Refresh Tasks'}
        </Button>
      </div>
      
      {tasks.length === 0 && !loading && (
        <div className="p-4 text-center text-gray-500">No tasks found.</div>
      )}

      <div className="grid grid-cols-1 gap-4">
        {tasks.map((task) => (
          <OutputContainer key={task.id}>
            <div className="flex flex-col gap-2 p-4">
              <div className="flex justify-between items-center">
                <span className="font-bold text-lg">Task #{task.id}</span>
                <span className={`px-2 py-1 rounded text-sm font-medium ${
                  task.status === TaskStatus.Open ? 'bg-green-100 text-green-800' :
                  task.status === TaskStatus.Completed ? 'bg-blue-100 text-blue-800' :
                  task.status === TaskStatus.Failed ? 'bg-red-100 text-red-800' :
                  'bg-yellow-100 text-yellow-800'
                }`}>
                  {task.status}
                </span>
              </div>
              
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm mt-2">
                <div className="flex flex-col">
                  <span className="text-gray-500">Creator</span>
                  <span className="truncate" title={task.creator}>{task.creator}</span>
                </div>
                <div className="flex flex-col">
                  <span className="text-gray-500">Reward</span>
                  <span>{formatAmount({ input: task.reward_amount, digits: 4, showLastNonZeroDecimal: true })} EGLD</span>
                </div>
                <div className="flex flex-col">
                  <span className="text-gray-500">Workers</span>
                  <span>{task.submissions_count} / {task.max_workers}</span>
                </div>
                <div className="flex flex-col">
                  <span className="text-gray-500">Docker Image</span>
                  <span className="truncate" title={task.docker_image_uri}>{task.docker_image_uri}</span>
                </div>
              </div>
              
              {task.status === TaskStatus.Open && (
                <div className="mt-4 pt-4 border-t border-gray-100 flex flex-col md:flex-row gap-2">
                    <Button 
                        onClick={() => copyWorkerCommand(task)}
                        className="flex-1 flex items-center justify-center gap-2 bg-gray-200 text-gray-800 hover:bg-gray-300"
                    >
                        <FontAwesomeIcon icon={faTerminal} />
                        {copiedId === task.id ? 'Command Copied!' : 'Copy Command'}
                    </Button>
                    
                    <Button 
                        onClick={() => handleParticipate(task)}
                        disabled={processingId === task.id}
                        className="flex-1 flex items-center justify-center gap-2"
                    >
                        <FontAwesomeIcon icon={faPlay} />
                        {processingId === task.id ? 'Processing...' : 'Run Worker'}
                    </Button>
                </div>
              )}
            </div>
          </OutputContainer>
        ))}
      </div>
    </div>
  );
};
