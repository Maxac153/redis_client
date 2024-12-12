import http.client

from src.delete_dumps_folder import delete_all_files_in_directory


class RedisClient:
    __dump_folder_path = './scripts/python/dumps/'

    def __init__(
            self,
            client_host: str,
            client_port: int
    ) -> None:
        self.conn = http.client.HTTPConnection(client_host, client_port)


    def download(self) -> None:
        delete_all_files_in_directory(self.__dump_folder_path)
        keys = []
        with open('./scripts/python/resources/download_keys.txt', 'r', encoding='utf-8') as f:
            keys = [line.strip() for line in f]

        for key in keys:
            self.conn.request(
                'GET',
                f'/downloadDumpKey?key={key}'
            )
            response = self.conn.getresponse()
            dump_data = response.read()
            with open(f'./scripts/python/dumps/{key}.dump', 'wb') as file:
                file.write(dump_data)
                print(f'Download Dump: {key}.dump')


    def upload(self) -> None:
        keys = []
        with open('./scripts/python/resources/upload_keys.txt', 'r', encoding='utf-8') as f:
            keys = [line.strip() for line in f]

        for key in keys:
            with open(f'./scripts/python/dumps/{key}.dump', 'rb') as file:
                headers = {
                    'Content-Type': 'application/octet-stream',
                }

                self.conn.request(
                    'POST',
                    f'/uploadDumpKey?key_name={key}',
                    file.read(),
                    headers
                )
                print('Upload response: ' + str(self.conn.getresponse().read()))


    def change_ttl(self, ttl) -> None:
        keys = []
        with open('./scripts/python/resources/change_ttl_keys.txt', 'r', encoding='utf-8') as f:
            keys = [line.strip() for line in f]

        for key in keys:
            self.conn.request(
                'PATCH',
                f'/changeTtl?key={key}&ttl={ttl}'
            )
            
            print('Change ttl response: ' + str(self.conn.getresponse().read()))


    def reset(self) -> None:
        keys = []
        with open('./scripts/python/resources/reset_keys.txt', 'r', encoding='utf-8') as f:
            keys = [line.strip() for line in f]

        for key in keys:
            self.conn.request(
                'DELETE',
                f'/resetKey?key={key}'
            )
            
            print('Reset key response: ' + str(self.conn.getresponse().read()))


    def close(self) -> None:
        self.conn.close()
