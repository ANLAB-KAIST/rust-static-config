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
        stage('Check') {
            steps {
                sh 'cargo check'
                sh 'cargo fmt --all -- --check'
                sh 'cargo clippy -- -D warnings'
            }
        }
        stage('Build') {
            steps {
                sh 'cargo build'
                sh 'cargo build --release'
            }
        }
        stage('Test') {
            steps {
                sh 'cargo test -- --nocapture --test-threads=1'
                sh 'cargo test --release -- --nocapture --test-threads=1'
            }
        }
    }
}