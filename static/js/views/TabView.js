export class TabView {
    constructor() {
        this.tabLinks = document.getElementsByClassName('tab-links');
        this.tabContent = document.getElementsByClassName('tab-content');

        this.readResponseBody = document.getElementById('readResponseBody');
        this.addResponseBody = document.getElementById('addResponseBody');
        this.ttlResponseBody = document.getElementById('ttlResponseBody');
        this.renameResponseBody = document.getElementById('renameKeyResponseBody');
        this.resetResponseBody = document.getElementById('resetKeyResponseBody');
        this.downloadDumpResponseBody = document.getElementById('downloadDumpResponseBody');
        this.uploadDumpResponseBody = document.getElementById('uploadDumpResponseBody');

        this.readModeBlock = document.getElementById('readModeBlock');
        this.readKeepBlock = document.getElementById('readKeepBlock');

        this.addFieldBlock = document.getElementById('addFieldBlock');
        this.addModeBlock = document.getElementById('addModeBlock');
    }

    renderTab(event, tabName, tabContentName) {
        const tab = document.getElementById(tabContentName);

        if (tab.style.display == 'block') {
            tab.style.display = 'none';
            event.currentTarget.className = event
                .currentTarget
                .className
                .replace(' active', '');

            localStorage.setItem('tabOpen', null);
        } else {
            let i;

            for (i = 0; i < this.tabContent.length; i++)
                this.tabContent[i].style.display = 'none';

            for (i = 0; i < this.tabLinks.length; i++)
                this.tabLinks[i].className = this.tabLinks[i].className.replace(' active', '');

            tab.style.display = 'block';
            event.currentTarget.className += ' active';
            localStorage.setItem('tabOpen', tabName);
        }
    }

    renderReadTab(response) {
        this.readResponseBody.value = response;
    }

    rednderReadInput(readTypeKey) {
        if (readTypeKey === 'List') {
            this.readModeBlock.style.display = 'flex';
            this.readKeepBlock.style.display = 'flex';
        } else if (readTypeKey === 'Hash') {
            this.readModeBlock.style.display = 'none';
            this.readKeepBlock.style.display = 'none';
        }
    }

    rednderAddInput(addTypeKey) {
        if (addTypeKey === 'List') {
            this.addFieldBlock.style.display = 'none';
            this.addModeBlock.style.display = 'flex';
        } else if (addTypeKey === 'Hash') {
            this.addFieldBlock.style.display = 'flex';
            this.addModeBlock.style.display = 'none';
        }
    }

    renderAddTab(response) {
        this.addResponseBody.value = response;
    }

    renderTtlTab(response) {
        this.ttlResponseBody.value = response;
    }

    renderRenameTab(response) {
        this.renameResponseBody.value = response;
    }

    renderResetKeyTab(response) {
        this.resetResponseBody.value = response;
    }

    renderResetAllKeyTab(response) {
        this.resetResponseBody.value = response;
    }

    renderDownloadTab(response) {
        this.downloadDumpResponseBody.value = response;
    }

    renderUploadTab(response) {
        this.uploadDumpResponseBody.value = response;
    }
}