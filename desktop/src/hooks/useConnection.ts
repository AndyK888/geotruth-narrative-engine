import { useState, useEffect, useCallback } from 'react';
import { apiClient, type HealthResponse } from '../api';

type ConnectionStatus = 'online' | 'offline' | 'checking';

interface UseConnectionResult {
  status: ConnectionStatus;
  health: HealthResponse | null;
  error: string | null;
  checkConnection: () => Promise<void>;
}

/**
 * Hook to manage API connection status
 */
export function useConnection(): UseConnectionResult {
  const [status, setStatus] = useState<ConnectionStatus>('checking');
  const [health, setHealth] = useState<HealthResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  const checkConnection = useCallback(async () => {
    setStatus('checking');
    setError(null);

    try {
      const response = await apiClient.health();
      setHealth(response);
      setStatus('online');
    } catch (err) {
      setHealth(null);
      setStatus('offline');
      setError(err instanceof Error ? err.message : 'Connection failed');
    }
  }, []);

  useEffect(() => {
    checkConnection();

    // Check connection periodically
    const interval = setInterval(checkConnection, 30000);
    return () => clearInterval(interval);
  }, [checkConnection]);

  return { status, health, error, checkConnection };
}

export default useConnection;
