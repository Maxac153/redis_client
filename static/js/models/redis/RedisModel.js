export class RedisModel {
    constructor() {
        this.redisStatus = null;
    }

    setRedisStatus(redisStatus) {
        this.redisStatus = redisStatus;
    }

    getRedisStatus() {
        return this.redisStatus;
    }
}