https://github.com/kubernetes/cri-api/tree/3506b127d6560bd9ef4ff656afd4a3ddf753a3d5/pkg/apis/runtime/v1

```shell
go build .
./inspect -container 5a9cfd687e7f2 | grep cgroup
```