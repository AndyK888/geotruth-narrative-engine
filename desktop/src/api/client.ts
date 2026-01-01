/**
 * API Client for communicating with the GeoTruth backend
 */

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8000';

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
    private baseUrl: string;
    private correlationId: string | null = null;

    constructor(baseUrl: string = API_URL) {
        this.baseUrl = baseUrl;
    }

    private async request<T>(
        method: string,
        path: string,
        body?: unknown
    ): Promise<T> {
        const headers: Record<string, string> = {
            'Content-Type': 'application/json',
        };

        if (this.correlationId) {
            headers['X-Correlation-ID'] = this.correlationId;
        }

        const response = await fetch(`${this.baseUrl}${path}`, {
            method,
            headers,
            body: body ? JSON.stringify(body) : undefined,
        });

        // Capture correlation ID from response
        this.correlationId = response.headers.get('X-Correlation-ID');

        if (!response.ok) {
            const error = await response.json().catch(() => ({}));
            throw new Error(error.detail || `API error: ${response.status}`);
        }

        return response.json();
    }

    async health(): Promise<HealthResponse> {
        return this.request<HealthResponse>('GET', '/v1/health');
    }

    async enrich(request: EnrichRequest): Promise<EnrichResponse> {
        return this.request<EnrichResponse>('POST', '/v1/enrich', request);
    }

    async enrichBatch(
        points: EnrichRequest[]
    ): Promise<{ results: EnrichResponse[] }> {
        return this.request('POST', '/v1/enrich_batch', { points });
    }

    getCorrelationId(): string | null {
        return this.correlationId;
    }
}

export const apiClient = new ApiClient();
export default apiClient;
