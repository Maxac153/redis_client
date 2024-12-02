export class TabAddModel {
    constructor() {
        this.addTypeKey = '';
        this.redisKey = '';
        this.field = '';
        this.addMode = '';
        this.dataRequest = '';
        this.dataResponse = '';
    }

    setAddTypeKey(addTypeKey) {
        this.addTypeKey = addTypeKey;
    }

    setRedisKey(redisKey) {
        this.redisKey = redisKey;
    }

    setAddMode(addMode) {
        this.addMode = addMode;
    }

    setField(field) {
        this.field = field;
    }

    setDataRequest(dataRequest) {
        this.dataRequest = dataRequest;
    }

    setDataResponse(dataResponse) {
        this.dataResponse = dataResponse;
    }

    getAddTypeKey() {
        return this.addTypeKey;
    }

    getField() {
        return this.field;
    }

    getRedisKey() {
        return this.redisKey;
    }

    getAddMode() {
        return this.addMode;
    }

    getDataRequest() {
        return this.dataRequest;
    }

    getDataResponse() {
        return this.dataResponse;
    }
}