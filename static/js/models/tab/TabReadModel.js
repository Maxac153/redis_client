export class TabReadModel {
    constructor() {
        this.readTypeKey = '';
        this.redisKey = '';
        this.readMode = '';
        this.keepMode = '';
        this.dataResponse = '';
    }

    setReadTypeKey(readTypeKey) {
        this.readTypeKey = readTypeKey;
    }

    setRedisKey(redisKey) {
        this.redisKey = redisKey;
    }

    setReadMode(readMode) {
        this.readMode = readMode;
    }

    setKeepMode(keepMode) {
        this.keepMode = keepMode;
    }

    setDataResponse(dataResponse) {
        this.dataResponse = dataResponse;
    }

    getReadTypeKey() {
        return this.readTypeKey
    }

    getRedisKey() {
        return this.redisKey;
    }

    getReadMod() {
        return this.readMode;
    }

    getKeepMode() {
        return this.keepMode;
    }

    getDataResponse() {
        return this.dataResponse;
    }
}