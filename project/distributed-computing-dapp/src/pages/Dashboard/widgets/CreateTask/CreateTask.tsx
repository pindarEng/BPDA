import { useState } from 'react';
import { Button } from 'components/Button';
import { OutputContainer } from 'components/OutputContainer';
import { 
    Address, 
    SmartContractTransactionsFactory, 
    TransactionsFactoryConfig,
    U32Value, 
    BytesValue 
} from '@multiversx/sdk-core';
import { contractAddress } from 'config';
import { signAndSendTransactions } from 'helpers/signAndSendTransactions';
import { useGetAccount, useGetNetworkConfig, parseAmount } from 'lib';

export const CreateTask = () => {
  const { address } = useGetAccount();
  const { network } = useGetNetworkConfig();
  const [dockerImage, setDockerImage] = useState('');
  const [inputData, setInputData] = useState('');
  const [maxWorkers, setMaxWorkers] = useState('');
  const [reward, setReward] = useState('');

  const handleSubmit = async () => {
    if (!address) return;

    try {
      const factoryConfig = new TransactionsFactoryConfig({ chainID: network.chainId });
      const factory = new SmartContractTransactionsFactory({ config: factoryConfig });

      const transaction = await factory.createTransactionForExecute(
        new Address(address),
        {
            contract: new Address(contractAddress),
            function: 'postTask',
            gasLimit: BigInt(20000000),
            nativeTransferAmount: BigInt(parseAmount(reward, 18)),
            arguments: [
                BytesValue.fromUTF8(dockerImage),
                BytesValue.fromUTF8(inputData),
                new U32Value(Number(maxWorkers))
            ]
        }
      );
      
      await signAndSendTransactions({
          transactions: [transaction],
          transactionsDisplayInfo: {
              processingMessage: 'Creating task...',
              errorMessage: 'An error has occurred during task creation',
              successMessage: 'Task created successfully'
          }
      });
    } catch (error) {
      console.error('Error creating task:', error);
    }
  };

  return (
    <OutputContainer>
        <div className="flex flex-col gap-4 p-4">
            <h2 className="text-xl font-bold">Create New Task</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="flex flex-col gap-2">
                    <label className="text-sm font-medium text-gray-500">Docker Image URI</label>
                    <input 
                        className="p-2 border rounded bg-gray-50 text-black focus:outline-none focus:ring-2 focus:ring-blue-500" 
                        value={dockerImage} 
                        onChange={(e) => setDockerImage(e.target.value)} 
                        placeholder="e.g. ubuntu:latest"
                    />
                </div>
                <div className="flex flex-col gap-2">
                    <label className="text-sm font-medium text-gray-500">Input Data URI</label>
                    <input 
                        className="p-2 border rounded bg-gray-50 text-black focus:outline-none focus:ring-2 focus:ring-blue-500" 
                        value={inputData} 
                        onChange={(e) => setInputData(e.target.value)} 
                        placeholder="e.g. https://example.com/data.json"
                    />
                </div>
                <div className="flex flex-col gap-2">
                    <label className="text-sm font-medium text-gray-500">Max Workers</label>
                    <input 
                        className="p-2 border rounded bg-gray-50 text-black focus:outline-none focus:ring-2 focus:ring-blue-500" 
                        type="number" 
                        value={maxWorkers} 
                        onChange={(e) => setMaxWorkers(e.target.value)}
                        min="1"
                    />
                </div>
                <div className="flex flex-col gap-2">
                    <label className="text-sm font-medium text-gray-500">Reward (EGLD)</label>
                    <input 
                        className="p-2 border rounded bg-gray-50 text-black focus:outline-none focus:ring-2 focus:ring-blue-500" 
                        type="number" 
                        step="0.01"
                        value={reward} 
                        onChange={(e) => setReward(e.target.value)}
                        min="0"
                    />
                </div>
            </div>
            <div className="flex justify-end mt-2">
                <Button 
                    onClick={handleSubmit} 
                    disabled={!dockerImage || !inputData || !maxWorkers || !reward}
                    className="w-full md:w-auto"
                >
                    Create Task
                </Button>
            </div>
        </div>
    </OutputContainer>
  );
};
