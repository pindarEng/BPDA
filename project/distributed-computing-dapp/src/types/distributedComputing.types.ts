export enum TaskStatus {
  Open = 'Open',
  InVerification = 'InVerification',
  Completed = 'Completed',
  Failed = 'Failed'
}

export interface Task {
  id: number;
  creator: string;
  docker_image_uri: string;
  input_data_uri: string;
  reward_amount: string;
  max_workers: number;
  submissions_count: number;
  status: TaskStatus;
}
