# Bitcoin Core Dockerizing/Running on K8s

## (Local) Deployment

Using [skaffold](https://skaffold.dev/) to simplify the build/deploy pipeline by delegating the tagging of docker images and generating of Kubernetes manifests with the said tags.

To build and auto deploy into a local cluster sharing the same Docker daemon (ie Docker for Desktop), install skaffold and with your Kubernetes context set to the local cluster `docker-desktop` run:

```sh
skaffold run --tail
```

And you should see the full build and deploy of all resources to your locally connected cluster.

```sh
skaffold build
```

To just build the docker image and tag with the current commit.

Alternatively you can use vanilla docker commands:

```sh
docker build -t bitcoin-core --target production .
```

## CI

Using github actions and skaffold to build and publish to the Github container registry under the repo.

See `.github/workflows/main.yml` for details.

Auto publishes docker image on each commit to main to https://github.com/jracollins/experimenting/pkgs/container/bitcoin-core

### Build Notes

Keys are embedded in the repo from https://github.com/bitcoin/bitcoin/blob/master/contrib/builder-keys/keys.txt @ https://github.com/bitcoin/bitcoin/commit/8d869a7bb55a181358031575211810416aadda70 rather than fetching on demand.
