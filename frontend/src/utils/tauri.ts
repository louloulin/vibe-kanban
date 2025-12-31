/**
 * Tauri API utilities for desktop app
 */

// Check if running in Tauri desktop environment
export const isTauri = () => {
  return window.__TAURI__ !== undefined;
};

// Lazy load Tauri APIs
export const getTauriAPI = async () => {
  if (!isTauri()) {
    throw new Error('Not running in Tauri environment');
  }

  const { invoke } = await import('@tauri-apps/api/core');
  const { listen } = await import('@tauri-apps/api/event');
  const { open } = await import('@tauri-apps/api/shell');

  return { invoke, listen, open };
};

// Tauri command wrappers
export const tauriCommands = {
  // Health check
  healthCheck: async () => {
    const { invoke } = await getTauriAPI();
    return invoke('health_check');
  },

  // Deployment
  getDeploymentInfo: async () => {
    const { invoke } = await getTauriAPI();
    return invoke('get_deployment_info');
  },

  initializeDeployment: async () => {
    const { invoke } = await getTauriAPI();
    return invoke('initialize_deployment');
  },

  // Projects
  getProjects: async () => {
    const { invoke } = await getTauriAPI();
    return invoke('get_projects');
  },

  getProject: async (id: string) => {
    const { invoke } = await getTauriAPI();
    return invoke('get_project', { id });
  },

  createProject: async (name: string, description?: string) => {
    const { invoke } = await getTauriAPI();
    return invoke('create_project', { name, description });
  },

  updateProject: async (
    id: string,
    name?: string,
    description?: string
  ) => {
    const { invoke } = await getTauriAPI();
    return invoke('update_project', { id, name, description });
  },

  deleteProject: async (id: string) => {
    const { invoke } = await getTauriAPI();
    return invoke('delete_project', { id });
  },

  // Tasks
  getTasks: async (projectId: string) => {
    const { invoke } = await getTauriAPI();
    return invoke('get_tasks', { projectId });
  },

  getTask: async (id: string) => {
    const { invoke } = await getTauriAPI();
    return invoke('get_task', { id });
  },

  createTask: async (
    projectId: string,
    title: string,
    description?: string
  ) => {
    const { invoke } = await getTauriAPI();
    return invoke('create_task', { projectId, title, description });
  },

  updateTask: async (
    id: string,
    title?: string,
    description?: string,
    status?: string
  ) => {
    const { invoke } = await getTauriAPI();
    return invoke('update_task', { id, title, description, status });
  },

  deleteTask: async (id: string) => {
    const { invoke } = await getTauriAPI();
    return invoke('delete_task', { id });
  },

  // Executors
  getExecutors: async () => {
    const { invoke } = await getTauriAPI();
    return invoke('get_executors');
  },

  getExecutorConfig: async (name: string) => {
    const { invoke } = await getTauriAPI();
    return invoke('get_executor_config', { name });
  },

  updateExecutorConfig: async (
    name: string,
    config: Record<string, unknown>
  ) => {
    const { invoke } = await getTauriAPI();
    return invoke('update_executor_config', { name, config });
  },

  // Filesystem
  readFile: async (path: string) => {
    const { invoke } = await getTauriAPI();
    return invoke('read_file', { path });
  },

  writeFile: async (path: string, content: string) => {
    const { invoke } = await getTauriAPI();
    return invoke('write_file', { path, content });
  },

  listDirectory: async (path: string) => {
    const { invoke } = await getTauriAPI();
    return invoke('list_directory', { path });
  },

  // Config
  getConfig: async () => {
    const { invoke } = await getTauriAPI();
    return invoke('get_config');
  },

  updateConfig: async (config: Record<string, unknown>) => {
    const { invoke } = await getTauriAPI();
    return invoke('update_config', { config });
  },

  // Events
  listen: async (event: string, callback: (payload: unknown) => void) => {
    const { listen } = await getTauriAPI();
    return listen(event, (event) => callback(event.payload));
  },
};

// Platform detection
export const getPlatform = () => {
  if (isTauri()) {
    return 'desktop';
  }
  return 'web';
};

// Window controls (desktop only)
export const windowControls = {
  minimize: async () => {
    if (isTauri()) {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      const window = getCurrentWindow();
      window.minimize();
    }
  },

  maximize: async () => {
    if (isTauri()) {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      const window = getCurrentWindow();
      window.toggleMaximize();
    }
  },

  close: async () => {
    if (isTauri()) {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      const window = getCurrentWindow();
      window.close();
    }
  },

  hide: async () => {
    if (isTauri()) {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      const window = getCurrentWindow();
      window.hide();
    }
  },

  show: async () => {
    if (isTauri()) {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      const window = getCurrentWindow();
      window.show();
      window.setFocus(true);
    }
  },
};
