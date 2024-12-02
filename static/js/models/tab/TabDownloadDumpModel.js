export class TabDownloadDumpModel {
    constructor() {
        this.dumpKey = '';
        this.dataResponse = '';
    }

    setDumpKey(dumpKey) {
        this.dumpKey = dumpKey;
    }

    setDataResponse(dataRequest) {
        this.dataResponse = dataRequest;
    }

    getDumpKey() {
        return this.dumpKey;
    }

    getDataResponse() {
        return this.dataResponse;
    }
}