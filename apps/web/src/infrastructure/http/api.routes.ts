export const API = {
  identity: {
    login: 'api/v1/identity/login',
    register: 'api/v1/identity/register',
    refresh: 'api/v1/identity/refresh',
    logout: 'api/v1/identity/logout',
    me: 'api/v1/identity/me',
    users: 'api/v1/identity/users',
    tenant: 'api/v1/identity/tenant',
    apiKeys: 'api/v1/identity/api-keys',
  },
  tickets: {
    list: 'api/v1/tickets',
    create: 'api/v1/tickets',
    get: (id: string) => `api/v1/tickets/${id}`,
    updateStatus: (id: string) => `api/v1/tickets/${id}/status`,
    messages: (id: string) => `api/v1/tickets/${id}/messages`,
    approveAi: (id: string) => `api/v1/tickets/${id}/approve-ai`,
    rejectAi: (id: string) => `api/v1/tickets/${id}/reject-ai`,
  },
  knowledge: {
    list: 'api/v1/knowledge',
    create: 'api/v1/knowledge',
    search: 'api/v1/knowledge/search',
    delete: (id: string) => `api/v1/knowledge/${id}`,
  },
  realtime: {
    ticket: (id: string) => `api/v1/realtime/tickets/${id}`,
    system: 'api/v1/realtime/system',
  },
} as const
