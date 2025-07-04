import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../AuthContext';
import '../styles/Auth.css';
import { AuthApi, Configuration } from '../ts-client';

const API_BASE = '/api';

const Profile: React.FC = () => {
  const { user, logout, setUser } = useAuth();
  const navigate = useNavigate();
  const [editing, setEditing] = useState(false);
  const [firstname, setFirstname] = useState(user?.firstname || '');
  const [lastname, setLastname] = useState(user?.lastname || '');
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  const [loading, setLoading] = useState(false);

  console.log('Profile user:', user);

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  if (!user) return null;

  const getInitials = (firstname: string, lastname: string) => {
    return `${firstname.charAt(0)}${lastname.charAt(0)}`.toUpperCase();
  };

  const handleEdit = () => {
    setEditing(true);
    setError('');
    setSuccess('');
  };

  const handleCancel = () => {
    setEditing(false);
    setFirstname(user.firstname);
    setLastname(user.lastname);
    setError('');
    setSuccess('');
  };

  const handleSave = async () => {
    setLoading(true);
    setError('');
    setSuccess('');
    try {
      const api = new AuthApi(new Configuration({ basePath: API_BASE }));
      const resp = await api.profilePatch({
        firstname,
        lastname,
      } as any);
      if (resp && resp.data) {
        setUser(resp.data);
        setSuccess('Profile updated!');
        setEditing(false);
      }
    } catch (err: any) {
      setError(err.response?.data?.error || 'Failed to update profile');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="profile-container">
      <div className="profile-box">
        {/* Welcome Animation */}
        <div className="welcome-animation">
          <span role="img" aria-label="wave" className="wave-emoji">👋</span>
          <h1 className="fade-in">Welcome, {user.firstname}!</h1>
        </div>
        <div className="profile-header">
          <div className="profile-avatar">
            {getInitials(user.firstname, user.lastname)}
          </div>
          <div className="profile-info">
            <h2>{`${user.firstname} ${user.lastname}`}</h2>
            <p>{user.role}</p>
          </div>
        </div>
        <div className="profile-details">
          <div className="profile-field">
            <strong>Email</strong>
            <span>{user.email}</span>
          </div>
          <div className="profile-field">
            <strong>First Name</strong>
            {editing ? (
              <input
                type="text"
                value={firstname}
                onChange={e => setFirstname(e.target.value)}
                minLength={2}
                required
              />
            ) : (
              <span>{user.firstname}</span>
            )}
          </div>
          <div className="profile-field">
            <strong>Last Name</strong>
            {editing ? (
              <input
                type="text"
                value={lastname}
                onChange={e => setLastname(e.target.value)}
                minLength={2}
                required
              />
            ) : (
              <span>{user.lastname}</span>
            )}
          </div>
          <div className="profile-field">
            <strong>Role</strong>
            <span>{user.role}</span>
          </div>
        </div>
        {editing ? (
          <div className="profile-edit-actions">
            <button onClick={handleSave} disabled={loading} className="save-button">
              {loading ? 'Saving...' : 'Save'}
            </button>
            <button onClick={handleCancel} disabled={loading} className="cancel-button">
              Cancel
            </button>
          </div>
        ) : (
          <button onClick={handleEdit} className="edit-button">
            Edit Profile
          </button>
        )}
        <button onClick={handleLogout} className="logout-button">
          Logout
        </button>
        {error && <div className="error">{error}</div>}
        {success && <div className="success">{success}</div>}
      </div>
    </div>
  );
};

export default Profile;
