export class TableView {
    units = ['B', 'K', 'M', 'G', 'T']

    constructor() {
        this.redisConnectedClients = document.getElementById('connectedClients');
        this.redisTotalMemoryUsage = document.getElementById('totalMemoryUsage');
        this.redisKeyList = document.getElementById('tableBody');

        this.preLoader = document.getElementById('preLoader');
        this.content = document.getElementById('content');

        this.interval = 0;
        this.inputRefreshRate = document.getElementById('inputRefreshRate');
        this.checkboxRefresh = document.getElementById('checkboxRefresh');

        this.pageInfo = document.getElementById('pageInfo');
        this.btnPaginationPrev = document.getElementById('btnPaginationPrev');
        this.btnPaginationNext = document.getElementById('btnPaginationNext');

        this.textareaJsonValidation = document.getElementById('jsonValidation')
    }

    convertBytes(bytes) {
        if (bytes < 1024) {
            return `${bytes} B`;
        }

        let unitIndex = 0;
        let value = bytes;

        while (value >= 1024 && unitIndex < this.units.length - 1) {
            value /= 1024;
            ++unitIndex;
        }

        return `${value.toFixed(2)} ${this.units[unitIndex]}`;
    }

    convertTtlToTime(totalSeconds) {
        if (totalSeconds === -1) {
            return '∞';
        }
        else {
            totalSeconds = Math.floor(totalSeconds / 1000);
            let hours = Math.floor(totalSeconds / 3600);
            let minutes = Math.floor((totalSeconds % 3600) / 60);
            let seconds = totalSeconds % 60;
            return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
        }
    }

    validateInputDigit = (
        input,
        numberDigits,
        min,
        max
    ) => {
        input.value = input.value.replace(/[^0-9]/g, '');

        if (input.value.length > numberDigits) {
            input.value = input.value.slice(0, numberDigits);
        }

        let value = parseInt(input.value, 10);

        if (isNaN(value) || value < min) {
            input.value = min;
        } else if (value >= max) {
            input.value = max;
        }
    }

    scrolling(e) {
        e.preventDefault();
        let value = parseInt(this.inputRefreshRate.value, 10);

        if (e.deltaY < 0) {
            ++value;
        }
        else if (e.deltaY > 0) {
            --value;
        }

        if (value !== null) {
            this.inputRefreshRate.value = value;
            this.validateInputDigit(this.inputRefreshRate, 2, 1, 60);
        }
    }

    generateTableRow(index, key, len, expectedNumberOfRecords, memoryUsage, ttl) {
        let newRow = document.createElement('tr')
        if (expectedNumberOfRecords === '-') {
        } else if (expectedNumberOfRecords > len) {
            newRow.className = 'low-value'
        } else if (expectedNumberOfRecords <= len) {
            newRow.className = 'high-value'
        }

        let cell1 = document.createElement('td')
        cell1.className = 'index-key'
        cell1.textContent = index

        let cell2 = document.createElement('td')
        cell2.className = 'name-key'
        cell2.textContent = key

        let cell3 = document.createElement('td')
        cell3.className = 'number-records-key'
        cell3.textContent = len

        let cell4 = document.createElement('td')
        cell4.className = 'expected-number-records-key'
        cell4.textContent = expectedNumberOfRecords

        let cell5 = document.createElement('td')
        cell5.className = 'memory-usage-key'
        cell5.textContent = this.convertBytes(memoryUsage)

        let cell6 = document.createElement('td')
        cell6.className = 'ttl-key'
        cell6.textContent = this.convertTtlToTime(ttl)

        newRow.append(cell1, cell2, cell3, cell4, cell5, cell6)
        return newRow
    }

    renderRefresh(e, loadDataFromRedis) {
        clearInterval(this.intervalId);
        let interval = parseInt(this.inputRefreshRate.value, 10);

        if (this.checkboxRefresh.checked) {
            this.inputRefreshRate.disabled = false;
            this.inputRefreshRate.addEventListener('wheel', this.scrolling(e));
            this.intervalId = setInterval(() => loadDataFromRedis(), interval * 1000);
        } else {
            this.inputRefreshRate.disabled = true;
            this.inputRefreshRate.removeEventListener('wheel', this.scrolling);
        }
    }

    renderPreLoader() {
        preLoader.style.opacity = '0'
        preLoader.style.display = 'none'
        content.style.display = 'block'
    }

    renderPaginationButtons(dataLength, rowsPerPage, currentPage) {
        let numberPages = Math.ceil(dataLength / rowsPerPage)
        this.btnPaginationPrev.disabled = currentPage === 1;
        this.btnPaginationNext.disabled = currentPage === numberPages || (numberPages === 0);
        this.pageInfo.textContent = `Page ${currentPage} of ${Math.ceil(dataLength / rowsPerPage) || 1}`;
    }

    renderTable(redisStatus, searchKey, rowsPerPage) {
        this.redisKeyList.innerHTML = '';
        this.redisConnectedClients.textContent = `Connected Clients: ${redisStatus.connected_clients}`;
        this.redisTotalMemoryUsage.textContent = `Total Memory Usage: ${redisStatus.total_memory_usage.replace(/(\d+\.?\d*)([a-zA-Z])/g, '$1 $2')}`;

        let jsonValidation = {};
        try {
            jsonValidation = JSON.parse(this.textareaJsonValidation.value);
        } catch { }

        let keys = redisStatus.keys;
        Object.keys(jsonValidation).forEach(key => {
            if (key.includes(searchKey.value) && !keys.includes(key)) {
                keys.push(key);
                if (redisStatus.statuses.length < rowsPerPage) {
                    redisStatus.statuses.push({
                        key,
                        len: 0,
                        expectedNumberOfRecords: jsonValidation[key],
                        memory_usage: 0,
                        ttl: -1
                    });
                }
            }
        });

        redisStatus.statuses.forEach((status, index) => {
            this.redisKeyList.appendChild(
                this.generateTableRow(
                    index + 1,
                    status.key,
                    status.len,
                    jsonValidation[status.key] || '-',
                    status.memory_usage,
                    status.ttl
                )
            );
        });
    }
}