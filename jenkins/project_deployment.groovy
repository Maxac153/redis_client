#!groovy

// Скачивание образов (.rs, .tar) в пространство Jenkins
void downloadingImagesGenToJenkins(DeployJson json) {
    for (Images image in json.images) {
        sshagent(credentials: [env.CREDENTIAL]) {
            sh "scp ${env.USERNAME}@${json.genImages}:${json.pathImages + image.imageName} ./"
        }
        stash name: image.imageName, includes: image.imageName
    }
}

// Загрузка образа где нужно развернуть приложение
void uploadImageJenkinsToGen(String image, String gen, String genPathFolder) {
    node(gen) {
        unstash image
        sh "sudo mkdir -p ${genPathFolder}"
        sh "sudo mv ${image} ${genPathFolder}"
    }
}

// Создание скрипта для deploy
static String createDeployScript(String pathFolder, String[] commands) {
    String script = "cd ${pathFolder};"
    for (String command in commands)
        script += "\n" + command

    return script
}

// Остановка старого образа
def stopOldImage(String image, String gen, String genPathFolder) {
    node(gen) {
        sh "cd ${genPathFolder};\n" +
            "pgid=\$(pgrep -f ${image}) && sudo kill \$pgid || echo \"Процесс не найден!\"\n" +
            "sudo rm ${image} || echo \"Файл не найден!\""
    }
}

// Запуск скрипта
def deployScript(String gen, String script) {
    node(gen) {
        sh script
    }
}

class Images {
    public String type
    public String imageName
    public String gen
    public String[] commands
}

class DeployJson {
    public String genImages
    public String pathImages
    public Images[] images
}

pipeline {
    agent any
    parameters {
        text(name: 'JSON', description: 'JSON for deployment', defaultValue: '')
    }

    environment {
        // Креды пользователя для подключения по ssh и scp
        // !!! Перед запуском джобы подставить параметры !!!
        USERNAME = ''
        CREDENTIAL = ''
    }

    stages {
        stage('DELETE OLD IMAGES JENKINS') {
            steps {
                deleteDir()
            }
        }

        stage('DEPLOY IMAGES') {
            when {
                expression { params.JSON != '' }
            }
            steps {
                script {
                    def scripts = []
                    def DEPLOY_PARAM = readJSON text: params.JSON
                    DeployJson json = new DeployJson(DEPLOY_PARAM)

                    // Скачиваем нужные образы c генератора
                    downloadingImagesGenToJenkins(json)

                    // Создаём скрипты для деплоя
                    for (Images image in json.images) {
                        String script = createDeployScript(json.pathImages, image.commands)
                        echo "${image.imageName}: ${script}"
                        scripts.add([
                                type     : image.type,
                                imageName: image.imageName,
                                script   : script,
                                gen      : image.gen,
                                pathImage: json.pathImages
                        ])
                    }

                    // Запуск развёртывания
                    scripts.eachWithIndex { script, index ->
                        stage("${index + 1}. Image: ${script.imageName} Gen: ${script.gen}") {
                            if (script.type == '.rs')
                                stopOldImage(script.imageName, script.gen, script.pathImage)

                            // Загружаем образ с Jenkins на генератор где будем разворачивать
                            uploadImageJenkinsToGen(script.imageName, script.gen, script.pathImage)

                            // Запускаем скрипт для развёртывания приложения
                            deployScript(script.gen, script.script)
                        }
                    }
                }
            }
        }
    }
}
