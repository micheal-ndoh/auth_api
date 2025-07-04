import React, { createContext, useContext, useState, useEffect, useCallback, ReactNode } from 'react';
import { AuthApi, Configuration, LoginRequest, LoginResponse, User, ProtectedApi, RegisterRequest } from './ts-client';

const API_BASE = '/api';

interface AuthContextType {
  user: User | null;
  accessToken: string | null;
  refreshToken: string | null;
  login: (data: LoginRequest) => Promise<void>;
  logout: () => void;
  register: (data: RegisterRequest) => Promise<void>;
  loading: boolean;
  setUser: React.Dispatch<React.SetStateAction<User | null>>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const useAuth = () => {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error('useAuth must be used within AuthProvider');
  return ctx;
};

let logoutTimer: NodeJS.Timeout | null = null;

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [user, setUser] = useState<User | null>(() => {
    const stored = localStorage.getItem('user');
    return stored ? JSON.parse(stored) : null;
  });
  const [accessToken, setAccessToken] = useState<string | null>(() => {
    return localStorage.getItem('accessToken');
  });
  const [refreshToken, setRefreshToken] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const getApiConfig = useCallback((token?: string) => {
    return new Configuration({
      basePath: API_BASE,
      accessToken: token || accessToken || undefined,
    });
  }, [accessToken]);

  const authApi = new AuthApi(getApiConfig());

  useEffect(() => {
    console.log('AuthContext accessToken:', accessToken);
  }, [accessToken]);

  useEffect(() => {
    console.log('AuthContext user:', user);
  }, [user]);

  // Persist user and token
  useEffect(() => {
    if (user) {
      localStorage.setItem('user', JSON.stringify(user));
    } else {
      localStorage.removeItem('user');
    }
  }, [user]);

  useEffect(() => {
    if (accessToken) {
      localStorage.setItem('accessToken', accessToken);
    } else {
      localStorage.removeItem('accessToken');
    }
  }, [accessToken]);

  // Logout
  const logout = useCallback(() => {
    setAccessToken(null);
    setRefreshToken(null);
    setUser(null);
    localStorage.removeItem('accessToken');
    localStorage.removeItem('user');
    if (logoutTimer) clearTimeout(logoutTimer);
  }, []);

  // Fetch user profile (now after logout is defined)
  const fetchUserProfile = useCallback(async (token: string) => {
    try {
      const protectedApi = new ProtectedApi(getApiConfig(token));
      const response = await protectedApi.userRoute({
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      if (response.data) {
        setUser(response.data);
        return response.data;
      }
      throw new Error('Failed to fetch user data');
    } catch (error) {
      console.error('Failed to fetch user profile:', error);
      logout();
      throw error;
    }
  }, [getApiConfig, logout]);

  // Helper to set session and timer
  const startSession = useCallback(async (access: string, refresh: string) => {
    setAccessToken(access);
    setRefreshToken(refresh);
    if (logoutTimer) clearTimeout(logoutTimer);
    logoutTimer = setTimeout(() => {
      logout();
    }, 10 * 60 * 1000); // 10 minutes
    
    // Fetch user profile after setting the token
    return await fetchUserProfile(access);
  }, [fetchUserProfile]);

  // Login
  const login = async (data: LoginRequest) => {
    setLoading(true);
    try {
      const resp = await authApi.login(data);
      console.log('Login response:', resp);
      if (resp.data && resp.data.token) {
        await startSession(resp.data.token, ''); // No refreshToken in response
      } else {
        throw new Error('Invalid login response');
      }
    } finally {
      setLoading(false);
    }
  };

  // Register
  const register = async (data: RegisterRequest) => {
    setLoading(true);
    try {
      const resp = await authApi.register(data);
      console.log('Register response:', resp);
    } finally {
      setLoading(false);
    }
  };

  // Refresh token
  /*
  const refresh = useCallback(async () => {
    if (!refreshToken) return;
    try {
      const resp = await authApi.refreshToken({ refreshToken: { refreshToken } });
      if (resp.accessToken && resp.refreshToken && resp.user) {
        startSession(resp.accessToken, resp.refreshToken, resp.user);
      } else {
        logout();
      }
    } catch {
      logout();
    }
  }, [refreshToken, startSession, logout]);
  */

  // Optionally, auto-refresh token before expiry (every 9.5 minutes)
  /*
  useEffect(() => {
    if (accessToken && refreshToken) {
      const interval = setInterval(() => {
        refresh();
      }, 9.5 * 60 * 1000);
      return () => clearInterval(interval);
    }
  }, [accessToken, refreshToken, refresh]);
  */

  // Cleanup timer on unmount
  useEffect(() => {
    return () => {
      if (logoutTimer) clearTimeout(logoutTimer);
    };
  }, []);

  return (
    <AuthContext.Provider value={{ user, accessToken, refreshToken, login, logout, register, loading, setUser }}>
      {children}
    </AuthContext.Provider>
  );
}; 