export class ApiRedisService {
    constructor(baseURL) {
        this.baseURL = baseURL;
    }

    downloadDump = (blob, fileName) => {
        let url = URL.createObjectURL(blob);
        let a = document.createElement('a');

        a.href = url;
        a.download = fileName;
        document.body.appendChild(a);
        a.click();

        document.body.removeChild(a);
        URL.revokeObjectURL(url);
    }

    formatCurrentDate = () => {
        let now = new Date();
        let day = String(now.getDate()).padStart(2, '0');
        let month = String(now.getMonth() + 1).padStart(2, '0');
        let year = now.getFullYear();

        return `${day}_${month}_${year}`;
    }

    async httpQueryStatusDataRedis(searchKey, typeKey, lowerLimit, upperLimit) {
        try {
            const response = await fetch(`${this.baseURL}statusJson?search_key=${encodeURIComponent(searchKey)}&type_key=${typeKey}&lower_limit=${lowerLimit}&upper_limit=${upperLimit}`);

            if (!response.ok) {
                const errorData = await response.json();
                throw new Error(JSON.stringify(errorData));
            }

            return await response.json();
        } catch (error) {
            console.error('Error Redis (Status Json):', error);
            throw error;
        }
    }

    async httpQueryReadListDataRedis(key, readMode) {
        try {
            const response = await fetch(`${this.baseURL}readList?key=${encodeURIComponent(key)}&read_mode=${readMode}`);
            return await response.json();
        } catch (error) {
            console.error('Error Redis (Read Data):', error);
            throw error;
        }
    }

    async httpQueryReadHashDataRedis(key) {
        try {
            const response = await fetch(`${this.baseURL}readHash?key=${encodeURIComponent(key)}`);
            return await response.json();
        } catch (error) {
            console.error('Error Redis (Read Data):', error);
            throw error;
        }
    }

    async httpQueryAddListDataRedis(key, addMode, data) {
        try {
            const response = await fetch(`${this.baseURL}addList?key=${encodeURIComponent(key)}&add_mode=${addMode}`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: data,
            });

            return await response.json();
        } catch (error) {
            console.error('Error Redis (Add Data):', error);
            throw error;
        }
    }

    async httpQueryAddHashDataRedis(key, field, data) {
        try {
            const response = await fetch(`${this.baseURL}addHash?key=${encodeURIComponent(key)}&field=${encodeURIComponent(field)}`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: data,
            });

            return await response.json();
        } catch (error) {
            console.error('Error Redis (Add Data):', error);
            throw error;
        }
    }

    async httpQueryChangeTtlRedis(key, tll) {
        try {
            const response = await fetch(`${this.baseURL}changeTtl?key=${encodeURIComponent(key)}&ttl=${tll}`, {
                method: 'PATCH',
            });

            return await response.json();
        } catch (error) {
            console.error('Error Redis (Change Ttl):', error);
            throw error;
        }
    }

    async httpQueryRenameKeyRedis(oldNameKey, newNameKey) {
        try {
            const response = await fetch(`${this.baseURL}renameKey?old_name_key=${encodeURIComponent(oldNameKey)}&new_name_key=${encodeURIComponent(newNameKey)}`, {
                method: 'PATCH',
            });

            return await response.json();
        } catch (error) {
            console.error('Error Redis (Rename Key):', error);
            throw error;
        }
    }

    async httpQueryResetKeyRedis(key) {
        try {
            const response = await fetch(`${this.baseURL}resetKey?key=${encodeURIComponent(key)}`, {
                method: 'DELETE',
            });

            return await response.json();
        } catch (error) {
            console.error('Error Redis (Reset Key):', error);
            throw error;
        }
    }

    async httpQueryResetAllKeysRedis() {
        try {
            const response = await fetch(`${this.baseURL}resetAllKeys`, {
                method: 'DELETE',
            });

            return await response.json();
        } catch (error) {
            console.error('Error Redis (Reset All Keys):', error);
            throw error;
        }
    }

    async httpQueryDownloadDumpKeyRedis(key) {
        try {
            const response = await fetch(`${this.baseURL}downloadDumpKey?key=${encodeURIComponent(key)}`);
            const contentType = response.headers.get("content-type");
            if (contentType && contentType.includes("application/json")) {
                return response.json();
            }
            return response.blob().then(blob => {
                this.downloadDump(blob, `${key}_${this.formatCurrentDate()}.dump`)
                return {
                    status: 'OK',
                    massage: 'Successful dump download, key',
                    data: ''
                };
            });
        } catch (error) {
            console.error('Error Redis (Download Dump Key):', error);
            throw error;
        }
    }

    async httpQueryDownloadDumpAllKeysRedis() {
        try {
            const response = await fetch(`${this.baseURL}downloadDumpAllKeys`);
            const contentType = response.headers.get("content-type");
            if (contentType && contentType.includes("application/json")) {
                return response.json();
            }
            return response.blob().then(blob => {
                this.downloadDump(blob, `dump_all_keys_${this.formatCurrentDate()}.rdb`)
                return {
                    status: 'OK',
                    massage: 'Successful dump download, key',
                    data: ''
                };
            });
        } catch (error) {
            console.error('Error Redis (Download Dump All Keys):', error);
            throw error;
        }
    }

    async httpQueryUploadDumpKeyRedis(file) {
        const cleanedFileName = file.name.replace(/(_\d{2}_\d{2}_\d{4}(\s\(\d+\))?)?\.\w+$/, '');
        try {
            const response = await fetch(`${this.baseURL}uploadDumpKey?key_name=${cleanedFileName}`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/octet-stream'
                },
                body: file
            });

            return await response.json();
        } catch (error) {
            console.error('Error Redis (Upload Dump Key):', error);
            throw error;
        }
    }

    async httpQueryUploadDumpAllKeysRedis(file) {
        try {
            const response = await fetch(`${this.baseURL}uploadDumpAllKeys`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/octet-stream'
                },
                body: file
            });

            return await response.json();
        } catch (error) {
            console.error('Error Redis (Upload Dump All Keys):', error);
            throw error;
        }
    }
}