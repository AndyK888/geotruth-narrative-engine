/**
 * API Client for communicating with the GeoTruth backend via Tauri
 */

import { invoke } from '@tauri-apps/api/core';

export interface HealthResponse {
  status: string;
  version: string;
  environment: string;
  services: {
    database: string;
    redis: string;
    valhalla: string;
  };
}

export interface EnrichRequest {
  lat: number;
  lon: number;
  timestamp?: string;
  heading_deg?: number;
  fov_deg?: number;
}

export interface POI {
  id: string;
  name: string;
  category: string;
  distance_m: number;
  bearing_deg: number;
  in_fov: boolean;
  confidence: number;
  facts?: Record<string, unknown>;
}

export interface EnrichResponse {
  location: {
    lat: number;
    lon: number;
    matched?: {
      lat: number;
      lon: number;
      road_name: string;
      road_class: string;
    };
  };
  context: {
    country: string;
    state: string;
    county: string;
    timezone: string;
    elevation_m: number;
  };
  pois: POI[];
}

class ApiClient {
  // correlationId is kept for interface compatibility but not used in Tauri IPC
  private correlationId: string | null = null;

  constructor() {
    // No base URL needed for Tauri
  }

  async health(): Promise<HealthResponse> {
    try {
      const version = await invoke<string>('get_version');
      // Check implicit health by version command success.
      // We could also call check_api_connection if we maintained that concept,
      // but in Monolith, if backend responds, it's healthy.

      return {
        status: 'ok',
        version: version,
        environment: 'production', // or check is_development
        services: {
          database: 'duckdb', // Native
          redis: 'dashmap', // Native
          valhalla: 'georust', // Native
        },
      };
    } catch (e) {
      console.error('Health check failed', e);
      throw new Error(`Health check failed: ${e}`);
    }
  }

  async enrich(request: EnrichRequest): Promise<EnrichResponse> {
    try {
      return await invoke<EnrichResponse>('enrich', { request });
    } catch (e) {
      console.error('Enrichment failed', e);
      throw e;
    }
  }

  async enrichBatch(points: EnrichRequest[]): Promise<{ results: EnrichResponse[] }> {
    // Parallelize requests since we lack a batch command for now
    const results = await Promise.all(points.map((p) => this.enrich(p)));
    return { results };
  }

  getCorrelationId(): string | null {
    return this.correlationId;
  }
}

export const apiClient = new ApiClient();
export default apiClient;
