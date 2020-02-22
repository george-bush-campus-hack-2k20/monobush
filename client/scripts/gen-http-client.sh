#!/bin/sh

set -eux

api_client_path="/monobush/client/api-client"
swagger_path="/monobush/swagger.yaml"

apk add -U curl jq maven

curl https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/4.2.3/openapi-generator-cli-4.2.3.jar -o openapi-generator-cli.jar

mkdir -p ~/bin/openapitools
curl https://raw.githubusercontent.com/OpenAPITools/openapi-generator/master/bin/utils/openapi-generator-cli.sh > ~/bin/openapitools/openapi-generator-cli
chmod u+x ~/bin/openapitools/openapi-generator-cli
export PATH=$PATH:~/bin/openapitools/
openapi-generator-cli generate -g javascript -i $swagger_path -o $api_client_path
chown -R $HOST_USER_ID:$HOST_GROUP_ID $api_client_path
