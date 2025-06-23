import React from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../AuthContext';
import '../styles/Auth.css';

const Profile: React.FC = () => {
  const { user, logout } = useAuth();
  const navigate = useNavigate();

  console.log('Profile user:', user);

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  if (!user) return null;

  const getInitials = (firstname: string, lastname: string) => {
    return `${firstname.charAt(0)}${lastname.charAt(0)}`.toUpperCase();
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
            <span>{user.firstname}</span>
          </div>
          <div className="profile-field">
            <strong>Last Name</strong>
            <span>{user.lastname}</span>
          </div>
          <div className="profile-field">
            <strong>Role</strong>
            <span>{user.role}</span>
          </div>
        </div>
        <button onClick={handleLogout} className="logout-button">
          Logout
        </button>
      </div>
    </div>
  );
};

export default Profile;
