export class TabChangeTtlModel {
    constructor() {
        this.redisKey = '';
        this.hour = 0;
        this.min = 0;
        this.sec = 0;
        this.dataResponse = ''
    }

    setRedisKey(redisKey) {
        this.redisKey = redisKey;
    }

    setHour(hour) {
        this.hour = hour;
    }

    setMin(min) {
        this.min = min;
    }

    setSec(sec) {
        this.sec = sec;
    }

    setDataResponse(dataResponse) {
        this.dataResponse = dataResponse;
    }

    getRedisKey() {
        return this.redisKey;
    }

    getHour() {
        return this.hour;
    }

    getMin() {
        return this.min;
    }

    getSec() {
        return this.sec;
    }

    getDataResponse() {
        return this.dataResponse;
    }
}