import { RedisController } from './controllers/RedisController.js';

const app = () => {
    new RedisController();
};

document.addEventListener('DOMContentLoaded', app);