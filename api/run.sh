#!/bin/sh
export DEF_GATEWAY=`ip route | grep default | cut -d ' ' -f 3`
# export DATABASE_URL="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$DEF_GATEWAY:5432/cookies"
export DATABASE_URL="postgres://postgres:cookies@$DEF_GATEWAY:5432/cookies"
export ROCKET_DATABASES="{cookies={url=$DATABASE_URL}}"

./diesel migration run --locked-schema
./cookies