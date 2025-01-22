package main

import (
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/apimachinery/pkg/fields"
	"k8s.io/client-go/tools/cache"
)

func main() {

	client, err := orchestration.NewClient()

	configMap, er := client.Clientset().CoreV1().ConfigMaps(namespace).Get(context.Background(), configMapName, metav1.GetOptions{})
	if er != nil {
		c.log.Error("error fetching the configMap",
			zap.Error(er))
	}

	watchlist := cache.NewListWatchFromClient(clientset.CoreV1().RESTClient(), "pods", metav1.NamespaceAll, fields.OneTermEqualSelector("spec.nodeName", ""))

	_, controller := cache.NewInformer(
		watchlist,
		&corev1.Pod{},
		0, // Duration is set to 0 for now
		cache.ResourceEventHandlerFuncs{
			AddFunc: func(obj interface{}) {
				// Handle unscheduled pods (e.g., scheduling logic here)
			},
		},
	)
	stop := make(chan struct{})
	defer close(stop)
	go controller.Run(stop) // Keep the main thread running
	select {}


}

func schedulePodToNode(pod *corev1.Pod, nodeName string, clientset *kubernetes.Clientset) error {
    // Update the pod's nodeName to schedule it
    pod.Spec.NodeName = nodeName
    _, err := clientset.CoreV1().Pods(pod.Namespace).Update(context.Background(), pod, metav1.UpdateOptions{})
    return err
}

// AddFunc: func(obj interface{}) {
//        pod := obj.(*corev1.Pod)
//
//        // Placeholder for your scheduling algorithm
//        // This should determine the best node for the pod
//        nodeName := findBestNodeForPod(pod)    // Assuming findBestNodeForPod is a function you'll implement that returns a nodeName
//        if nodeName != "" {
//            err := schedulePodToNode(pod, nodeName, clientset)
//            if err != nil {
//                fmt.Printf("Failed to schedule pod %s to node %s: %v\n", pod.Name, nodeName, err)
//            }
//        }
//    },


