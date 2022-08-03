# No requirements for multi-platform so will stick to most common x86 throughout and simplicity
FROM ubuntu:22.10@sha256:4a260e959a8e5186f1915e46bfb7663165ebfe5a7dc3365c76f9e3fb8d3019e4 as fetcher

RUN apt-get update 
RUN apt-get install -y curl gpg

WORKDIR /bitcoin 

# Most steps explained from https://bitcoincore.org/en/download/

ADD https://bitcoincore.org/bin/bitcoin-core-22.0/bitcoin-22.0-x86_64-linux-gnu.tar.gz bitcoin-22.0-x86_64-linux-gnu.tar.gz
ADD https://bitcoincore.org/bin/bitcoin-core-22.0/SHA256SUMS SHA256SUMS
ADD https://bitcoincore.org/bin/bitcoin-core-22.0/SHA256SUMS.asc SHA256SUMS.asc

RUN sha256sum --ignore-missing --check SHA256SUMS

COPY keys.txt keys.txt

RUN gpg --keyserver hkps://keys.openpgp.org --refresh-keys

RUN while read fingerprint keyholder_name; do gpg --keyserver hkps://keys.openpgp.org --recv-keys ${fingerprint}; done < ./keys.txt

# Issue with not all signatures provided having public key - grep number of successful signatures, will exit code 1 if count of good signatures is 0
RUN gpg --verify --quiet SHA256SUMS.asc 2>&1 | grep -c '^gpg: Good signature from'

RUN tar -xzvf bitcoin-22.0-x86_64-linux-gnu.tar.gz

FROM ubuntu:22.10@sha256:4a260e959a8e5186f1915e46bfb7663165ebfe5a7dc3365c76f9e3fb8d3019e4 as production

RUN useradd -ms /bin/bash bitcoin
WORKDIR /home/bitcoin
USER bitcoin

COPY --from=fetcher --chown=bitcoin /bitcoin/bitcoin-22.0/ ./bitcoin-22.0

WORKDIR /home/bitcoin/bitcoin-22.0

RUN ./bin/bitcoind -version

CMD [ "./bin/bitcoind", "-rpcallowip=0.0.0.0", "-rpcbind=0.0.0.0" ]
