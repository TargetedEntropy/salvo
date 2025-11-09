import axios from 'axios';
import { AnalysisRequest, AnalysisResponse } from './types';

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:3000';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

export const analyzeSalvage = async (request: AnalysisRequest): Promise<AnalysisResponse> => {
  const response = await api.post<AnalysisResponse>('/api/salvage/analyze', request);
  return response.data;
};

export const updateMarketPrices = async (typeIds: number[]): Promise<void> => {
  await api.post('/api/market/update', { type_ids: typeIds });
};

export default api;
