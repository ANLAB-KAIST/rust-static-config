pipeline {
    agent {
        docker { image 'rust:latest' }
    }
    stages {
        stage('Version') {
            steps {
                sh 'cargo --version'
                sh 'rustc --version'
                sh 'rustup component add rustfmt'
                sh 'rustup component add clippy'
            }
        }
    }
}