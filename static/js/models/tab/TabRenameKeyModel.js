export class TabRenameKeyModel {
    constructor() {
        this.oldNameKey = '';
        this.newNameKey = '';
        this.dataResponse = '';
    }

    setOldNameKey(oldNameKey) {
        this.oldNameKey = oldNameKey;
    }

    setNewNameKey(newNameKey) {
        this.newNameKey = newNameKey;
    }

    setDataResponse(dataResponse) {
        this.dataResponse = dataResponse;
    }

    getOldNameKey() {
        return this.oldNameKey;
    }

    getNewNameKey() {
        return this.newNameKey;
    }

    getDataResponse() {
        return this.dataResponse;
    }
}