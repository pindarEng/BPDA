import { EnvironmentsEnum } from 'lib/sdkDapp/sdkDapp.types';

export * from './sharedConfig';

export const API_URL = 'https://devnet-template-api.multiversx.com';
export const contractAddress =
  'erd1qqqqqqqqqqqqqpgqfgayg3ykmn6jluazdfhka02y3q9vjc8wnc0syn2dwx';
export const environment = EnvironmentsEnum.devnet;
export const sampleAuthenticatedDomains = [API_URL];
