package main

//     "encoding/json"
import (
    "context"
    "flag"
    "fmt"
    "log"

    "google.golang.org/protobuf/encoding/protojson"
    "google.golang.org/grpc"
    runtimev1 "k8s.io/cri-api/pkg/apis/runtime/v1"
    "google.golang.org/grpc/credentials/insecure"
)

func main() {
    // parse cli argument
    var containerID string
    flag.StringVar(&containerID, "container", "", "Container ID to inspect")
    flag.Parse()

    if containerID == "" {
        log.Fatalf("Please provide a container ID using the -container flag")
    }

    conn, err := grpc.Dial(
        "unix:///run/containerd/containerd.sock",
        grpc.WithTransportCredentials(insecure.NewCredentials()),
    )
    if err != nil {
        log.Fatalf("Failed to connect to container runtime: %v", err)
    }
    defer conn.Close()

    client := runtimev1.NewRuntimeServiceClient(conn)

    req := &runtimev1.ContainerStatusRequest{
        ContainerId: containerID,
        Verbose:     true,
    }

    resp, err := client.ContainerStatus(context.Background(), req)
    if err != nil {
        log.Fatalf("Failed to get container status: %v", err)
    }

    jsonData, err := json.MarshalIndent(resp, "", "")
    if err != nil {
        log.Fatalf("Failed to marshal response: %v", err)
    }

//  TODO: try the generated protobuf definitions api.pb.go
//         marshaler := protojson.MarshalOptions{
//                    Indent:          "  ",
//                    UseProtoNames:   true,
//                    EmitUnpopulated: true,
//            }
//
//    jsonData, err := marshaler.Marshal(resp)
//            if err != nil {
//                    fmt.Printf("Error marshaling to JSON: %v\n", err)
//                    return
//            }

    fmt.Println(string(jsonData))
}
