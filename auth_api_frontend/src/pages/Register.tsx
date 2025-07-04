import React, { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { useAuth } from '../AuthContext';
import { Role } from '../ts-client';
import '../styles/Auth.css';

const getPasswordStrength = (password: string) => {
  const minLength = password.length >= 8;
  const hasNumber = /[0-9]/.test(password);
  const hasSpecial = /[!@#$%^&*(),.?":{}|<>]/.test(password);
  return minLength && hasNumber && hasSpecial;
};

const Register: React.FC = () => {
  const { register, login, loading } = useAuth();
  const navigate = useNavigate();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [firstname, setFirstname] = useState('');
  const [lastname, setLastname] = useState('');
  const [error, setError] = useState('');
  const [showPassword, setShowPassword] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    if (!getPasswordStrength(password)) {
      setError('Password is too weak.');
      return;
    }
    try {
      await register({
        email,
        password,
        firstname,
        lastname,
      });
      // Automatically log the user in after registration
      await login({ email, password });
      navigate('/profile');
    } catch (err: any) {
      setError('Registration failed');
    }
  };

  const passwordStrong = getPasswordStrength(password);

  return (
    <div className="auth-container register-container">
      <div className="auth-box">
        <h2>Create Account</h2>
        <form className="auth-form" onSubmit={handleSubmit}>
          <div className="input-group">
            <input
              type="email"
              placeholder="Email"
              value={email}
              onChange={e => setEmail(e.target.value)}
              required
            />
          </div>
          <div className="input-group">
            <input
              type="text"
              placeholder="First Name"
              value={firstname}
              onChange={e => setFirstname(e.target.value)}
              required
            />
          </div>
          <div className="input-group">
            <input
              type="text"
              placeholder="Last Name"
              value={lastname}
              onChange={e => setLastname(e.target.value)}
              required
            />
          </div>
          <div className="input-group" style={{ position: 'relative' }}>
            <input
              type={showPassword ? 'text' : 'password'}
              placeholder="Password"
              value={password}
              onChange={e => setPassword(e.target.value)}
              required
              style={{
                borderColor: password.length === 0 ? '' : passwordStrong ? 'green' : 'red',
              }}
            />
            <span
              onClick={() => setShowPassword(s => !s)}
              style={{
                position: 'absolute',
                right: 10,
                top: '50%',
                transform: 'translateY(-50%)',
                cursor: 'pointer',
                userSelect: 'none',
                fontSize: 18,
              }}
              aria-label={showPassword ? 'Hide password' : 'Show password'}
              role="button"
            >
              {showPassword ? '🙈' : '👁️'}
            </span>
          </div>
          <div style={{ fontSize: 13, color: passwordStrong ? 'green' : 'red', marginBottom: 8 }}>
            Password must be at least 8 characters, contain a number and a special character.
          </div>
          <button type="submit" disabled={loading}>
            {loading ? 'Creating Account...' : 'Register'}
          </button>
        </form>
        {error && <div className="error">{error}</div>}
        <p>
          Already have an account? <Link to="/login" className="auth-link">Login</Link>
        </p>
      </div>
    </div>
  );
};

export default Register;
