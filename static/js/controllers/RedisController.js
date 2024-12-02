import { ApiRedisService } from '../services/redis/ApiRedisService.js';

import { RedisModel } from '../models/redis/RedisModel.js';
import { TabReadModel } from '../models/tab/TabReadModel.js';
import { TabAddModel } from '../models/tab/TabAddModel.js';
import { TabChangeTtlModel } from '../models/tab/TabChangeTtlModel.js';
import { TabRenameKeyModel } from '../models/tab/TabRenameKeyModel.js';
import { TabResetModel } from '../models/tab/TabResetModel.js'
import { TabDownloadDumpModel } from '../models/tab/TabDownloadDumpModel.js'
import { TabUploadDumpModel } from '../models/tab/TabUploadDumpModel.js';

import { TabView } from '../views/TabView.js';
import { TableView } from '../views/TableView.js';

export class RedisController {
    checkEmptyImput(input, filedName, render) {
        if (input.trim() === '') {
            if (typeof render === 'function') {
                render(
                    JSON.stringify({
                        status: 'KO',
                        message: `The (${filedName}) field must not be empty!`,
                        data: ''
                    }, null, 4)
                );
            } else {
                console.error('Render function is not defined or not a function');
            }
            return true;
        }
        return false;
    }

    loadingDataFromLocalStorage() {
        const storageLength = localStorage.length;
        for (let i = 0; i < storageLength; i++) {
            const key = localStorage.key(i);
            const value = localStorage.getItem(key);
            if (key == 'tabOpen') {
                document.getElementById(value).click();
            } else if (key == 'inputRefreshRate') {
                let pageElement = document.getElementById(key);
                pageElement.value = value;

                if (localStorage.getItem('checkboxRefresh') !== null) {
                    document.getElementById("checkboxRefresh").click();
                }
            } else if (key == 'readTypeKey') {
                let pageElement = document.getElementById(key);
                pageElement.value = value;
                this.tabReadModel.setReadTypeKey(value);
                this.choiceTypeReadKey();
            } else if (key == 'addTypeKey') {
                let pageElement = document.getElementById(key);
                pageElement.value = value;
                this.tabAddModel.setAddTypeKey(value);
                this.choiceTypeAddKey();
            } else {
                let pageElement = document.getElementById(key);
                pageElement.value = value;
            }
        }
    }

    updateStatus(textareaStatus, text, dots) {
        textareaStatus.textContent = text + '.'.repeat(dots)
        dots = (dots + 1) % 3
    }

    constructor() {
        this.redisModel = new RedisModel();

        this.tabReadModel = new TabReadModel();
        this.tabAddModel = new TabAddModel();
        this.tabChangeTtlModel = new TabChangeTtlModel();
        this.tabRenameKeyModel = new TabRenameKeyModel();
        this.tabResetModel = new TabResetModel();
        this.tabDownloadDumpModel = new TabDownloadDumpModel();
        this.tabUploadDumpModel = new TabUploadDumpModel();

        this.tabView = new TabView();
        this.tableView = new TableView();

        this.apiRedisService = new ApiRedisService(window.location.href);

        // Инициализация табов
        const tabs = [
            { tabName: 'tabRedisReadData', tabNameContent: 'redisReadData' },
            { tabName: 'tabRedisAddData', tabNameContent: 'redisAddData' },
            { tabName: 'tabRedisChangeTtl', tabNameContent: 'redisChangeTtl' },
            { tabName: 'tabRedisRenameKey', tabNameContent: 'redisRenameKey' },
            { tabName: 'tabRedisResetKey', tabNameContent: 'redisResetKey' },
            { tabName: 'tabRedisDownloadDump', tabNameContent: 'redisDownloadDump' },
            { tabName: 'tabRedisUploadDump', tabNameContent: 'redisUploadDump' },
        ];
        this.choiceTab(tabs);

        // Чтение из Redis
        const redisReadTypeKey = document.getElementById('readTypeKey');
        const redisReadKey = document.getElementById('readKey');
        const redisReadMode = document.getElementById('readMode');
        const redisKeepMode = document.getElementById('keepMode');
        this.responseTimeReadData = document.getElementById('responseTimeReadData');

        redisReadTypeKey.addEventListener('change', () => { localStorage.setItem('readTypeKey', redisReadTypeKey.value) });
        redisReadKey.addEventListener('change', () => { localStorage.setItem('readKey', redisReadKey.value) });
        redisReadMode.addEventListener('change', () => { localStorage.setItem('readMode', redisReadMode.value) });
        redisKeepMode.addEventListener('change', () => { localStorage.setItem('keepMode', redisKeepMode.value) });

        document.getElementById('readTypeKey').addEventListener('change', async () => {
            this.tabReadModel.setReadTypeKey(redisReadTypeKey.value);
            this.choiceTypeReadKey();
        });

        document.getElementById('btnReadData').addEventListener('click', async () => {
            if (!this.checkEmptyImput(
                redisReadKey.value,
                'Redis Key',
                (response) => this.tabView.renderReadTab(response)
            )) {
                this.tabReadModel.setRedisKey(redisReadKey.value);
                this.tabReadModel.setReadMode(redisReadMode.value);
                this.tabReadModel.setKeepMode(redisKeepMode.value);

                if (redisReadTypeKey.value === 'List') {
                    await this.readListDataRedis();
                } else if (redisReadTypeKey.value === 'Hash') {
                    await this.readHashDataRedis();
                }
                await this.loadDataRedis();
            }
        });

        // Добавление в Redis
        const redisAddTypeKey = document.getElementById('addTypeKey');
        const redisAddKey = document.getElementById('addKey');
        const redisField = document.getElementById('addField');
        const redisAddMode = document.getElementById('addMode');
        const redisAddDataRequest = document.getElementById('addRequestBody');
        this.responseTimeAddData = document.getElementById('responseTimeAddData');

        redisAddTypeKey.addEventListener('change', () => { localStorage.setItem('addTypeKey', redisAddTypeKey.value) });
        redisAddKey.addEventListener('change', () => { localStorage.setItem('addKey', redisAddKey.value) });
        redisField.addEventListener('change', () => { localStorage.setItem('addField', redisField.value) });
        redisAddMode.addEventListener('change', () => { localStorage.setItem('addMode', redisAddMode.value) });
        redisAddDataRequest.addEventListener('change', () => { localStorage.setItem('addRequestBody', redisAddDataRequest.value) });

        document.getElementById('addTypeKey').addEventListener('change', async () => {
            this.tabAddModel.setAddTypeKey(redisAddTypeKey.value);
            this.choiceTypeAddKey();
        });

        document.getElementById('btnAddData').addEventListener('click', async () => {
            this.tabAddModel.setField(redisField.value);
            this.tabAddModel.setAddTypeKey(redisAddTypeKey.value);
            if (!this.checkEmptyImput(
                redisAddKey.value,
                'Redis Key',
                (response) => this.tabView.renderAddTab(response)
            ) && !this.checkEmptyImput(
                redisAddDataRequest.value,
                'Request Body JSON',
                (response) => this.tabView.renderAddTab(response)
            ) && (!this.checkEmptyImput(
                this.tabAddModel.getField(),
                'Redis Field',
                (response) => this.tabView.renderAddTab(response)
            ) || this.tabAddModel.getAddTypeKey() === 'List')) {
                this.tabAddModel.setRedisKey(redisAddKey.value);
                this.tabAddModel.setAddMode(redisAddMode.value);
                this.tabAddModel.setDataRequest(redisAddDataRequest.value);

                if (this.tabAddModel.getAddTypeKey() === 'List') {
                    await this.addListDataRedis();
                } else if (this.tabAddModel.getAddTypeKey() === 'Hash') {
                    await this.addHashDataRedis();
                }
                await this.loadDataRedis();
            }
        });

        // Смена TTL
        const redisTtlKey = document.getElementById('ttlKey');
        const redisHour = document.getElementById('ttlHour');
        const redisMin = document.getElementById('ttlMin');
        const redisSec = document.getElementById('ttlSec');
        this.responseTimeTtlData = document.getElementById('responseTimeTtlData');

        redisTtlKey.addEventListener('change', () => { localStorage.setItem('ttlKey', redisTtlKey.value) });
        redisHour.addEventListener('change', () => { localStorage.setItem('ttlHour', redisHour.value) });
        redisMin.addEventListener('change', () => { localStorage.setItem('ttlMin', redisMin.value) });
        redisSec.addEventListener('change', () => { localStorage.setItem('ttlSec', redisSec.value) });

        redisHour.addEventListener('input', () => this.tableView.validateInputDigit(redisHour, 2, 0, 99))
        redisMin.addEventListener('input', () => this.tableView.validateInputDigit(redisMin, 2, 0, 59))
        redisSec.addEventListener('input', () => this.tableView.validateInputDigit(redisSec, 2, 0, 59))

        document.getElementById('btnTtl').addEventListener('click', async () => {
            if (!this.checkEmptyImput(
                redisTtlKey.value,
                'Redis Key',
                (response) => this.tabView.renderTtlTab(response)
            )) {
                this.tabChangeTtlModel.setRedisKey(redisTtlKey.value);
                this.tabChangeTtlModel.setHour(redisHour.value);
                this.tabChangeTtlModel.setMin(redisMin.value);
                this.tabChangeTtlModel.setSec(redisSec.value);
                await this.changeTtlRedis();
                await this.loadDataRedis();
            }
        })

        document.getElementById('btnDefaultTtl').addEventListener('click', async () => {
            if (!this.checkEmptyImput(
                redisTtlKey.value,
                'Redis Key',
                (response) => this.tabView.renderTtlTab(response)
            )) {
                this.tabChangeTtlModel.setRedisKey(redisTtlKey.value);
                this.tabChangeTtlModel.setHour(0);
                this.tabChangeTtlModel.setMin(0);
                this.tabChangeTtlModel.setSec(0);
                await this.changeTtlRedis();
                await this.loadDataRedis();
            }
        })

        // Смена имени
        const redisOldNameKey = document.getElementById('oldNameKey');
        const redisNewNameKey = document.getElementById('newNameKey');
        this.responseTimeRenameData = document.getElementById('responseTimeRenameData');

        redisOldNameKey.addEventListener('change', () => { localStorage.setItem('oldNameKey', redisOldNameKey.value) });
        redisNewNameKey.addEventListener('change', () => { localStorage.setItem('newNameKey', redisNewNameKey.value) });

        document.getElementById('btnRenameKey').addEventListener('click', async () => {
            if (!this.checkEmptyImput(
                redisOldNameKey.value,
                'Old Name Key',
                (response) => this.tabView.renderRenameTab(response)
            ) && !this.checkEmptyImput(
                redisNewNameKey.value,
                'New Name Key',
                (response) => this.tabView.renderRenameTab(response)
            )) {
                this.tabRenameKeyModel.setOldNameKey(redisOldNameKey.value);
                this.tabRenameKeyModel.setNewNameKey(redisNewNameKey.value);
                await this.renameKeyRedis();
                await this.loadDataRedis();
            }
        });

        // Удаление клача или всех данных из Redis
        const redisResetKey = document.getElementById('resetKey');
        this.responseTimeResetData = document.getElementById('responseTimeResetData');

        redisResetKey.addEventListener('change', () => { localStorage.setItem('resetKey', redisResetKey.value) });

        document.getElementById('btnResetKey').addEventListener('click', async () => {
            if (!this.checkEmptyImput(
                redisResetKey.value,
                'Reset Key',
                (response) => this.tabView.renderResetKeyTab(response)
            )) {
                this.tabResetModel.setRedisKey(redisResetKey.value);
                await this.resetKeyRedis();
                await this.loadDataRedis();
            }
        });

        document.getElementById('btnResetAllKey').addEventListener('click', async () => {
            let result = confirm('Are you sure you want to clear all data in Redis?');
            if (result) {
                await this.resetAllKeysRedis();
                await this.loadDataRedis();
            }
        });

        // Скачать дамп
        const redisDownloadKey = document.getElementById('downloadDumpKey');
        this.responseTimeDownloadData = document.getElementById('responseTimeDownloadData');

        redisDownloadKey.addEventListener('change', () => { localStorage.setItem('downloadDumpKey', redisDownloadKey.value) });

        document.getElementById('btnDownloadDumpKey').addEventListener('click', () => {
            if (!this.checkEmptyImput(
                redisDownloadKey.value,
                'Dump Key',
                (response) => this.tabView.renderDownloadTab(response)
            )) {
                this.tabDownloadDumpModel.setDumpKey(redisDownloadKey.value);
                this.downloadDumpKeyRedis();
            }
        });

        document.getElementById('btnDownloadDumpAllKeys').addEventListener('click', () => {
            this.downloadDumpAllKeysRedis();
        })

        // Загрузить дамп
        const redisUploadFile = document.getElementById('fileUploadDump');
        this.responseTimeUploadData = document.getElementById('responseTimeUploadData');

        document.getElementById('btnUploadDump').addEventListener('click', async () => {
            if (redisUploadFile.files.length === 0) {
                this.tabView.renderUploadTab(
                    JSON.stringify({
                        status: 'KO',
                        message: 'No file selected.',
                        data: ''
                    }, null, 4)
                );
                return;
            }
            this.tabUploadDumpModel.setFile(redisUploadFile.files[0]);
            await this.uploadDumpRedis();
            await this.loadDataRedis();
        })

        // Добавление прокрутки
        const refreshRate = document.getElementById('inputRefreshRate');

        refreshRate.addEventListener('input', () => this.tableView.validateInputDigit(refreshRate, 2, 1, 30));
        refreshRate.addEventListener('wheel', (e) => {
            this.refreshPage(e);
            localStorage.setItem('inputRefreshRate', refreshRate.value);
        });
        checkboxRefresh.addEventListener('change', (e) => this.refreshPage(e));

        this.search = document.getElementById('search');
        this.search.addEventListener('change', () => this.loadDataRedis());
        search.addEventListener('change', () => { localStorage.setItem('search', search.value) });

        this.searchTypeKey = document.getElementById('searchTypeKey');
        this.searchTypeKey.addEventListener('change', () => this.loadDataRedis());
        searchTypeKey.addEventListener('change', () => { localStorage.setItem('searchTypeKey', searchTypeKey.value) });

        // Модальное окно
        var modal = document.getElementById('modal')
        document.getElementById('validator').addEventListener('click', () => modal.style.display = 'block');
        document.getElementById('validatorHover').addEventListener('click', () => modal.style.display = 'block');
        document.getElementById('closeButton').addEventListener('click', () => {
            modal.style.display = 'none';
            this.loadDataRedis();
        })

        let jsonValidation = document.getElementById('jsonValidation');
        jsonValidation.addEventListener('change', () => { localStorage.setItem('jsonValidation', jsonValidation.value) });

        window.addEventListener('click', (event) => {
            if (event.target == modal) {
                modal.style.display = 'none';
                this.loadDataRedis();
            }
        });

        // Пагинация таблицы
        this.currentPage = 1;
        this.dataLength = 0;
        this.rowsPerPage = 20;

        document.getElementById('btnPaginationPrev').addEventListener('click', () => {
            if (this.currentPage > 1) {
                this.currentPage--
                this.loadDataRedis()
            }
        })

        document.getElementById('btnPaginationNext').addEventListener('click', () => {
            if (this.currentPage < Math.ceil(this.dataLength / this.rowsPerPage)) {
                this.currentPage++
                this.loadDataRedis()
            }
        })

        // Инцилизация страницы
        this.loadingDataFromLocalStorage();
        this.loadDataRedis();
    }

    choiceTab(tabs) {
        tabs.forEach(tab => {
            document
                .getElementById(tab.tabName)
                .addEventListener('click',
                    (event) => this.tabView.renderTab(
                        event,
                        tab.tabName,
                        tab.tabNameContent
                    )
                )
        })
    }

    async refreshPage(e) {
        localStorage.setItem('checkboxRefresh', checkboxRefresh.checked)
        this.tableView.renderRefresh(
            e,
            () => this.loadDataRedis()
        );
    }

    async loadDataRedis() {
        try {
            const statusJson = await this.apiRedisService.httpQueryStatusDataRedis(
                this.search.value,
                this.searchTypeKey.value,
                (this.currentPage - 1) * this.rowsPerPage,
                this.currentPage * this.rowsPerPage
            );
            this.redisModel.setRedisStatus(statusJson);
            this.tableView.renderTable(this.redisModel.getRedisStatus(), this.search, this.rowsPerPage);
            this.dataLength = statusJson.keys.length;
            this.tableView.renderPaginationButtons(this.dataLength, this.rowsPerPage, this.currentPage);
            this.tableView.renderPreLoader();
        } catch (error) {
            console.error(error);
        }
    }

    async readListDataRedis() {
        try {
            let startTime = performance.now();
            const dataReadResponse = await this.apiRedisService.httpQueryReadListDataRedis(
                this.tabReadModel.getRedisKey(),
                this.tabReadModel.getReadMod()
            );

            this.tabReadModel.setDataResponse(dataReadResponse);
            if (dataReadResponse.status === 'OK' && this.tabReadModel.getKeepMode() === 'TRUE') {
                this.tabView.renderReadTab(this.tabReadModel.getDataResponse().data);
                const dataAddResponse = await this.apiRedisService.httpQueryAddListDataRedis(
                    this.tabReadModel.getRedisKey(),
                    this.tabReadModel.getReadMod(),
                    this.tabReadModel.getDataResponse().data
                );

                if (dataAddResponse.status === 'KO') {
                    this.tabView.renderReadTab(dataAddResponse.message);
                }
            } else if (dataReadResponse.status === 'OK') {
                this.tabView.renderReadTab(this.tabReadModel.getDataResponse().data);
            } else if (dataReadResponse.status === 'KO') {
                this.tabView.renderReadTab(this.tabReadModel.getDataResponse().message);
            }
            this.responseTimeReadData.textContent = `Time: ${Math.round(performance.now() - startTime)} ms`;
        } catch (error) {
            console.error(error);
        }
    }

    async readHashDataRedis() {
        try {
            let startTime = performance.now();
            const dataReadResponse = await this.apiRedisService.httpQueryReadHashDataRedis(
                this.tabReadModel.getRedisKey(),
                this.tabReadModel.getReadMod()
            );

            this.tabReadModel.setDataResponse(dataReadResponse);
            this.tabView.renderReadTab(this.tabReadModel.getDataResponse().data);
            this.responseTimeReadData.textContent = `Time: ${Math.round(performance.now() - startTime)} ms`;
        } catch (error) {
            console.error(error);
        }
    }

    async choiceTypeReadKey() {
        this.tabView.rednderReadInput(this.tabReadModel.getReadTypeKey());
    }

    async addListDataRedis() {
        try {
            let startTime = performance.now();
            const dataAddResponse = await this.apiRedisService.httpQueryAddListDataRedis(
                this.tabAddModel.getRedisKey(),
                this.tabAddModel.getAddMode(),
                this.tabAddModel.getDataRequest()
            );

            this.tabAddModel.setDataResponse(dataAddResponse);
            this.tabView.renderAddTab(JSON.stringify(this.tabAddModel.getDataResponse(), null, 4));
            this.responseTimeAddData.textContent = `Time: ${Math.round(performance.now() - startTime)} ms`;
        } catch (error) {
            console.error(error);
        }
    }

    async addHashDataRedis() {
        try {
            let startTime = performance.now();
            const dataAddResponse = await this.apiRedisService.httpQueryAddHashDataRedis(
                this.tabAddModel.getRedisKey(),
                this.tabAddModel.getField(),
                this.tabAddModel.getDataRequest()
            );

            this.tabAddModel.setDataResponse(dataAddResponse);
            this.tabView.renderAddTab(JSON.stringify(this.tabAddModel.getDataResponse(), null, 4));
            this.responseTimeAddData.textContent = `Time: ${Math.round(performance.now() - startTime)} ms`;
        } catch (error) {
            console.error(error);
        }
    }

    async choiceTypeAddKey() {
        this.tabView.rednderAddInput(this.tabAddModel.getAddTypeKey());
    }

    async changeTtlRedis() {
        try {
            let startTime = performance.now();
            const ttl = parseInt(this.tabChangeTtlModel.getHour(), 10) * 3600 +
                parseInt(this.tabChangeTtlModel.getMin(), 10) * 60 +
                parseInt(this.tabChangeTtlModel.getSec(), 10);

            const dataChangeTtlResponse = await this.apiRedisService.httpQueryChangeTtlRedis(
                this.tabChangeTtlModel.getRedisKey(),
                ttl
            );

            this.tabChangeTtlModel.setDataResponse(dataChangeTtlResponse);
            this.tabView.renderTtlTab(JSON.stringify(this.tabChangeTtlModel.getDataResponse(), null, 4));
            this.responseTimeTtlData.textContent = `Time: ${Math.round(performance.now() - startTime)} ms`;
        } catch (error) {
            console.error(error);
        }
    }

    async renameKeyRedis() {
        try {
            let startTime = performance.now();
            const dataRenameKeyResponse = await this.apiRedisService.httpQueryRenameKeyRedis(
                this.tabRenameKeyModel.getOldNameKey(),
                this.tabRenameKeyModel.getNewNameKey()
            );

            this.tabRenameKeyModel.setDataResponse(dataRenameKeyResponse);
            this.tabView.renderRenameTab(JSON.stringify(this.tabRenameKeyModel.getDataResponse(), null, 4));
            this.responseTimeRenameData.textContent = `Time: ${Math.round(performance.now() - startTime)} ms`;
        } catch (error) {
            console.error(error)
        }
    }

    async resetKeyRedis() {
        try {
            let startTime = performance.now();
            const dataResetKeyResponse = await this.apiRedisService.httpQueryResetKeyRedis(this.tabResetModel.getRedisKey());
            this.tabResetModel.setDataResponse(dataResetKeyResponse);
            this.tabView.renderResetKeyTab(JSON.stringify(this.tabResetModel.getDataResponse(), null, 4));
            this.responseTimeResetData.textContent = `Time: ${Math.round(performance.now() - startTime)} ms`;
        } catch (error) {
            console.error(error);
        }
    }

    async resetAllKeysRedis() {
        try {
            let startTime = performance.now();
            const dataResetAllKeysResponse = await this.apiRedisService.httpQueryResetAllKeysRedis();
            this.tabView.renderResetAllKeyTab(JSON.stringify(dataResetAllKeysResponse, null, 4));
            this.responseTimeResetData.textContent = `Time: ${Math.round(performance.now() - startTime)} ms`;
        } catch (error) {
            console.error(error);
        }
    }

    async downloadDumpKeyRedis() {
        try {
            let startTime = performance.now();
            let intervalId = setInterval(
                () => this.updateStatus(this.responseTimeDownloadData, 'Download', 0),
                1000
            );

            const dataDownloadDumpResponse = await this.apiRedisService.httpQueryDownloadDumpKeyRedis(
                this.tabDownloadDumpModel.getDumpKey()
            );
            this.tabDownloadDumpModel.setDataResponse(dataDownloadDumpResponse);
            this.tabView.renderDownloadTab(JSON.stringify(this.tabDownloadDumpModel.getDataResponse(), null, 4));

            clearInterval(intervalId);
            this.responseTimeDownloadData.textContent = `Time: ${Math.round(performance.now() - startTime)} ms`;
        } catch (error) {
            console.error(error);
        }
    }

    async downloadDumpAllKeysRedis() {
        try {
            let startTime = performance.now();
            let intervalId = setInterval(
                () => this.updateStatus(this.responseTimeDownloadData, 'Download', 0),
                1000
            );

            const dataDownloadDumpAllKeyResponse = await this.apiRedisService.httpQueryDownloadDumpAllKeysRedis();
            this.tabView.renderDownloadTab(JSON.stringify(dataDownloadDumpAllKeyResponse, null, 4));

            clearInterval(intervalId);
            this.responseTimeDownloadData.textContent = `Time: ${Math.round(performance.now() - startTime)} ms`;
        } catch (error) {
            console.error(error);
        }
    }

    async uploadDumpRedis() {
        try {
            let file = this.tabUploadDumpModel.getFile();
            const fileExtension = file.name.split('.')[1]
            let dataUploadDumpResponse;

            let startTime = performance.now();
            let intervalId = setInterval(
                () => this.updateStatus(this.responseTimeUploadData, 'Download', 0),
                1000
            );

            if (fileExtension === 'dump') {
                dataUploadDumpResponse = await this.apiRedisService.httpQueryUploadDumpKeyRedis(file);
            } else if (fileExtension === 'rdb') {
                dataUploadDumpResponse = await this.apiRedisService.httpQueryUploadDumpAllKeysRedis(file);
            } else {
                this.tabView.renderUploadTab(JSON.stringify({
                    status: 'KO',
                    message: 'Incorrect file format (Expected format <.dump, .rdb>)',
                    data: ''
                }, null, 4));
                return
            }

            this.tabView.renderUploadTab(JSON.stringify(dataUploadDumpResponse, null, 4));

            clearInterval(intervalId);
            this.responseTimeUploadData.textContent = `Time: ${Math.round(performance.now() - startTime)} ms`;
        } catch (error) {
            console.error(error);
        }
    }
}