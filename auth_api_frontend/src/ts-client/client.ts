import { Configuration, AuthApi } from './api';

const api = new AuthApi(new Configuration({
    basePath: 'http://localhost:3000',
}));

async function login() {
    try {
        const response = await api.login({ email: 'admin', password: 'password' });
        console.log('Token:', response.data.token);
    } catch (error) {
        console.error('Login failed:', error);
    }
}

login(); 