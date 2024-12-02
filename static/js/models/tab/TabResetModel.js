export class TabResetModel {
    constructor() {
        this.redisKey = '';
        this.dataResponse = '';
    }

    setRedisKey(redisKey) {
        this.redisKey = redisKey;
    }

    setDataResponse(dataResponse) {
        this.dataResponse = dataResponse;
    }

    getRedisKey() {
        return this.redisKey;
    }

    getDataResponse() {
        return this.dataResponse;
    }
}