package utils

import "sync"

type Job func()
type JobGroup struct {
	wg   sync.WaitGroup
	jobs []Job
}

func (jg *JobGroup) Add(job Job) {
	jg.jobs = append(jg.jobs, job)
	jg.wg.Add(1)
}
func (jg *JobGroup) Run() {
	for _, job := range jg.jobs {
		go job()
	}
}

func (jg *JobGroup) RunAndWait() {
	for _, job := range jg.jobs {
		go func(job Job) {
			defer jg.wg.Done()
			job()
		}(job)
	}
	jg.wg.Wait()
}
