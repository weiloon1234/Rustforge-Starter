import axios, { type AxiosInstance, type InternalAxiosRequestConfig } from "axios";

export interface ApiClientConfig {
  /** Read the current access token (from auth store). */
  getToken: () => string | null;
  /** Attempt to refresh the session. Must throw on failure. */
  refreshAuth: () => Promise<void>;
  /** Called when refresh also fails — clear state and redirect. */
  onAuthFailure: () => void;
}

/**
 * Factory that creates an Axios instance with:
 * - Request interceptor: attaches `Authorization: Bearer <token>`
 * - Response interceptor: on 401, attempts a single token refresh then
 *   retries the original request. Concurrent 401s share one refresh call.
 */
export function createApiClient(config: ApiClientConfig): AxiosInstance {
  const api = axios.create();

  // ── Request: attach bearer token ────────────────────────
  api.interceptors.request.use((req) => {
    const token = config.getToken();
    if (token) {
      req.headers.Authorization = `Bearer ${token}`;
    }
    return req;
  });

  // ── Response: handle 401 → refresh → retry ─────────────
  let refreshPromise: Promise<void> | null = null;

  api.interceptors.response.use(
    (res) => res,
    async (error) => {
      const original = error.config as InternalAxiosRequestConfig & {
        _retry?: boolean;
      };

      if (error.response?.status !== 401 || original._retry) {
        return Promise.reject(error);
      }

      original._retry = true;

      // Deduplicate concurrent refresh calls
      if (!refreshPromise) {
        refreshPromise = config
          .refreshAuth()
          .finally(() => {
            refreshPromise = null;
          });
      }

      try {
        await refreshPromise;
      } catch {
        config.onAuthFailure();
        return Promise.reject(error);
      }

      // Retry with the new token
      const newToken = config.getToken();
      if (!newToken) {
        config.onAuthFailure();
        return Promise.reject(error);
      }

      original.headers.Authorization = `Bearer ${newToken}`;
      return api(original);
    },
  );

  return api;
}
