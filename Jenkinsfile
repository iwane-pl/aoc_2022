pipeline {
  agent none
  options {
    ansiColor('xterm')
    allowBrokenBuildClaiming()
    timestamps()
    disableConcurrentBuilds abortPrevious: true
    timeout(time: 2, unit: 'MINUTES')
    skipDefaultCheckout true
  }
  stages {
    stage('Remote integration tests - agent') {
      agent { label "linux" }
      steps {
        echo "Running ${env.STAGE_NAME} on ${env.NODE_NAME}"

        cleanWs()
        dir('starfish') {
          checkout scm
        }

        sh '''
        env

        tar cf sources.tar *
        '''

        // does stash preserve symlinks? NO
        // need to tar/untar manually
        stash(name: "pr-integration-remote-${CHANGE_ID}", includes: "sources.tar")

        script {
          def remoteAgent = env.NODE_NAME
          def parts = [:]

          for (int counter = 0; counter < 8; counter++) {
            // def is necessary, or else the closure (stage definition) will capture the LAST value of the loop counter
            // for all parts instead of all the values one by one (https://stackoverflow.com/a/67636424)
            def part = counter;
            parts["Integration part ${part}"] = {
              node("packages_tests") {
                withEnv(["part=${part}", "AGENT_HOST=${remoteAgent}", "PULL_REQUEST_ID=${env.CHANGE_ID}", "PULL_REQUEST_ACTUAL_COMMIT=${env.GIT_COMMIT}"]) {
                  stage("Remote integration part ${part}") {
                    echo "Running ${env.STAGE_NAME} on ${env.NODE_NAME}"
                    cleanWs()
                    unstash "pr-integration-remote-${CHANGE_ID}"

                      sh '''
                      tar xf sources.tar
                      sleep 180
                      '''
                  }
                }
              }
            }
          }
          throttle(['integration-tests-parts']) {
            parallel parts
          }
        }
      }
    }
  }
}
