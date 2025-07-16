import React, { useState } from "react";
import { useNavigate, Link, useLocation } from "react-router-dom";
import { useAuth } from "../AuthContext";
import "../styles/Auth.css";
import { GoogleLogin } from "@react-oauth/google";
import axios from "axios";

const Login: React.FC = () => {
  const { login, loading, loginWithToken } = useAuth();
  const navigate = useNavigate();
  const location = useLocation();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [showPassword, setShowPassword] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");

    try {
      await login({ email, password });
      // Get the intended destination or default to profile
      const from = location.state?.from?.pathname || "/profile";
      navigate(from, { replace: true });
    } catch (err: any) {
      console.error("Login error:", err);
      setError(err.response?.data?.message || "Invalid credentials");
    }
  };

  return (
    <div className="auth-container login-container">
      <div className="auth-box">
        <h2>Welcome Back</h2>
        <form className="auth-form" onSubmit={handleSubmit}>
          <div className="input-group">
            <input
              type="email"
              placeholder="Email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
              disabled={loading}
            />
          </div>
          <div className="input-group" style={{ position: "relative" }}>
            <input
              type={showPassword ? "text" : "password"}
              placeholder="Password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
              disabled={loading}
            />
            <span
              onClick={() => setShowPassword((s) => !s)}
              style={{
                position: "absolute",
                right: 10,
                top: "50%",
                transform: "translateY(-50%)",
                cursor: "pointer",
                userSelect: "none",
                fontSize: 18,
              }}
              aria-label={showPassword ? "Hide password" : "Show password"}
              role="button"
            >
              {showPassword ? "🙈" : "👁️"}
            </span>
          </div>
          <button
            type="submit"
            disabled={loading}
            className={loading ? "loading" : ""}
          >
            {loading ? "Logging in..." : "Login"}
          </button>
        </form>
        {/* Google Sign-In Button */}
        <div style={{ margin: "20px 0" }}>
          <GoogleLogin
            onSuccess={async (credentialResponse) => {
              if (credentialResponse.credential) {
                try {
                  const res = await axios.post(
                    "https://authapi-backend-01.up.railway.app/auth/google",
                    {
                      id_token: credentialResponse.credential,
                    }
                  );
                  if (res.data && res.data.token) {
                    await loginWithToken(res.data.token);
                    navigate("/profile");
                  } else {
                    setError("Google login failed: No token returned");
                  }
                } catch (err: any) {
                  setError("Google login failed");
                }
              }
            }}
            onError={() => {
              setError("Google login failed");
            }}
          />
        </div>
        {error && <div className="error">{error}</div>}
        <p>
          Don't have an account?{" "}
          <Link to="/register" className="auth-link">
            Register
          </Link>
        </p>
      </div>
    </div>
  );
};

export default Login;
