/**
 * API Adapter - Switches between HTTP and Tauri IPC
 */

import { isTauri, tauriCommands } from '@/utils/tauri';

// Types
export interface ApiResponse<T> {
  data: T;
  error?: string;
}

// Base URL for HTTP API
const HTTP_BASE_URL = import.meta.env.VITE_API_URL || '';

/**
 * API client that switches between HTTP and Tauri IPC
 */
class ApiClient {
  private useTauri: boolean;

  constructor() {
    this.useTauri = isTauri();
    console.log(`API Client initialized in ${this.useTauri ? 'Tauri' : 'HTTP'} mode`);
  }

  /**
   * Generic GET request
   */
  async get<T>(path: string, params?: Record<string, unknown>): Promise<T> {
    if (this.useTauri) {
      return this.getTauri(path, params);
    }
    return this.getHttp<T>(path, params);
  }

  /**
   * Generic POST request
   */
  async post<T>(path: string, body?: unknown): Promise<T> {
    if (this.useTauri) {
      return this.postTauri(path, body);
    }
    return this.postHttp<T>(path, body);
  }

  /**
   * Generic PUT request
   */
  async put<T>(path: string, body?: unknown): Promise<T> {
    if (this.useTauri) {
      return this.putTauri(path, body);
    }
    return this.putHttp<T>(path, body);
  }

  /**
   * Generic DELETE request
   */
  async delete<T>(path: string): Promise<T> {
    if (this.useTauri) {
      return this.deleteTauri(path);
    }
    return this.deleteHttp<T>(path);
  }

  // HTTP implementations

  private async getHttp<T>(
    path: string,
    params?: Record<string, unknown>
  ): Promise<T> {
    const url = new URL(`${HTTP_BASE_URL}${path}`);
    if (params) {
      Object.entries(params).forEach(([key, value]) => {
        if (value !== undefined) {
          url.searchParams.append(key, String(value));
        }
      });
    }

    const response = await fetch(url.toString());
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return response.json();
  }

  private async postHttp<T>(path: string, body?: unknown): Promise<T> {
    const response = await fetch(`${HTTP_BASE_URL}${path}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: body ? JSON.stringify(body) : undefined,
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return response.json();
  }

  private async putHttp<T>(path: string, body?: unknown): Promise<T> {
    const response = await fetch(`${HTTP_BASE_URL}${path}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: body ? JSON.stringify(body) : undefined,
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return response.json();
  }

  private async deleteHttp<T>(path: string): Promise<T> {
    const response = await fetch(`${HTTP_BASE_URL}${path}`, {
      method: 'DELETE',
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return response.json();
  }

  // Tauri implementations

  private async getTauri<T>(
    path: string,
    params?: Record<string, unknown>
  ): Promise<T> {
    // Map HTTP paths to Tauri commands
    const commandMap: Record<string, string> = {
      '/api/projects': 'get_projects',
      '/api/executors': 'get_executors',
    };

    const command = commandMap[path];
    if (!command) {
      throw new Error(`No Tauri command mapped for path: ${path}`);
    }

    return tauriCommands[command as keyof typeof tauriCommands] as T;
  }

  private async postTauri<T>(path: string, body?: unknown): Promise<T> {
    // Map HTTP paths to Tauri commands
    const commandMap: Record<string, string> = {
      '/api/projects': 'create_project',
      '/api/tasks': 'create_task',
    };

    const command = commandMap[path];
    if (!command) {
      throw new Error(`No Tauri command mapped for path: ${path}`);
    }

    return tauriCommands[command as keyof typeof tauriCommands] as T;
  }

  private async putTauri<T>(path: string, body?: unknown): Promise<T> {
    // Map HTTP paths to Tauri commands
    const commandMap: Record<string, string> = {
      '/api/projects': 'update_project',
      '/api/tasks': 'update_task',
    };

    const command = commandMap[path];
    if (!command) {
      throw new Error(`No Tauri command mapped for path: ${path}`);
    }

    return tauriCommands[command as keyof typeof tauriCommands] as T;
  }

  private async deleteTauri<T>(path: string): Promise<T> {
    // Map HTTP paths to Tauri commands
    const commandMap: Record<string, string> = {
      '/api/projects': 'delete_project',
      '/api/tasks': 'delete_task',
    };

    const command = commandMap[path];
    if (!command) {
      throw new Error(`No Tauri command mapped for path: ${path}`);
    }

    return tauriCommands[command as keyof typeof tauriCommands] as T;
  }
}

// Export singleton instance
export const apiClient = new ApiClient();
