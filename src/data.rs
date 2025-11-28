use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Publication {
    pub title: String,
    pub authors: String,
    pub venue: String,
    pub pdf_link: String,
    pub doi_link: String,
}

#[derive(Clone, PartialEq)]
pub struct ExperienceDetail {
    pub label: Option<String>,
    pub content: String,
    pub sub_details: Vec<ExperienceDetail>,
    pub internal_link: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct ExperienceItem {
    pub title: String,
    pub company: String,
    pub dates: String,
    pub description: String,
    pub details: Vec<ExperienceDetail>,
    pub tags: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub struct EducationItem {
    pub degree: String,
    pub university: String,
    pub dates: String,
}

#[derive(Clone, PartialEq)]
pub struct ResearchProject {
    pub title: String,
    pub description: String,
    pub highlights: Vec<String>,
    pub github_link: String,
    pub publications: Vec<ResearchPublication>,
}

#[derive(Clone, PartialEq)]
pub struct ResearchPublication {
    pub title: String,
    pub link: String,
}

pub fn get_featured_research() -> Signal<Vec<ResearchProject>> {
    use_signal(|| vec![
        ResearchProject {
            title: "Object-as-a-Service (OaaS)".to_string(),
            description: "Pioneered the OaaS paradigm and engineered \"Oparaca\" platforms using Java, Rust, and Python on Kubernetes/Knative. Implemented a unified object abstraction that encapsulates logic, state, and QoS requirements, utilizing Kafka/Zenoh for messaging and Minio (S3) for storage.".to_string(),
            highlights: vec![
                "Achieved 2.27x lower latency and 3.27x higher throughput compared to Knative by optimizing data locality and reducing database bottlenecks.".to_string(),
                "Extended the platform to the Edge-Cloud continuum, enabling developers to declaratively configure complex QoS requirements like high availability (e.g., 99.999%) and consistency levels (e.g, strong, eventual) without changing code.".to_string(),
                "Validated by a human study to boost developer productivity by 31%, reducing boilerplate code by 45% and configuration complexity by 90%.".to_string(),
            ],
            github_link: "https://github.com/hpcclab/OaaS".to_string(),
            publications: vec![
                ResearchPublication {
                    title: "IEEE Transactions on Computers (2025)".to_string(),
                    link: "https://arxiv.org/pdf/2408.04898".to_string(),
                },
                ResearchPublication {
                    title: "ACM SoCC 2024".to_string(),
                    link: "https://dl.acm.org/doi/pdf/10.1145/3698038.3698552".to_string(),
                },
                ResearchPublication {
                    title: "IEEE CLOUD 2023".to_string(),
                    link: "https://arxiv.org/pdf/2206.05361".to_string(),
                },
            ],
        },
        ResearchProject {
            title: "Action Engine".to_string(),
            description: "Developed a tool-augmented LLM system to automatically generate platform-agnostic FaaS workflows from natural language, featuring a novel \"Func Identifier\" and \"DAG Compiler\".".to_string(),
            highlights: vec![
                "Achieved a 100% pass rate for syntactically valid workflows and a 42% F1 score in function selection for complex scenarios, significantly outperforming baseline methods.".to_string(),
            ],
            github_link: "https://github.com/hpcclab/action_engine".to_string(),
            publications: vec![
                ResearchPublication {
                    title: "Future Generation Computer Systems (2026)".to_string(),
                    link: "https://doi.org/10.1016/j.future.2025.107947".to_string(),
                },
            ],
        },
    ])
}

pub fn get_publications() -> Signal<Vec<Publication>> {
    use_signal(|| vec![
        Publication {
            title: "Object as a Service: Simplifying Cloud-Native Development through Serverless Object Abstraction".to_string(),
            authors: "Lertpongrujikorn, Pawissanutt, and Mohsen Amini Salehi".to_string(),
            venue: "IEEE Transactions on Computers, 2025, accepted in Oct. 2025, In press.".to_string(),
            pdf_link: "https://arxiv.org/pdf/2408.04898".to_string(),
            doi_link: "https://doi.org/10.1109/TC.2025.3623602".to_string(),
        },
        Publication {
            title: "Action Engine: Automatic Workflow Generation in FaaS".to_string(),
            authors: "Esashi, Akiharu, Pawissanutt Lertpongrujikorn, Shinji Kato, and Mohsen Amini Salehi".to_string(),
            venue: "Future Generation Computer Systems (2026): 107947.".to_string(),
            pdf_link: "https://arxiv.org/pdf/2411.19485".to_string(),
            doi_link: "https://doi.org/10.1016/j.future.2025.107947".to_string(),
        },
        Publication {
            title: "Streamlining Cloud-Native Application Development and Deployment with Robust Encapsulation".to_string(),
            authors: "Lertpongrujikorn, Pawissanutt, Hai Duc Nguyen, and Mohsen Amini Salehi".to_string(),
            venue: "Proceedings of the 2024 ACM Symposium on Cloud Computing.".to_string(),
            pdf_link: "https://dl.acm.org/doi/pdf/10.1145/3698038.3698552".to_string(),
            doi_link: "https://doi.org/10.1145/3698038.3698552".to_string(),
        },
        Publication {
            title: "Tutorial: Object as a Service (OaaS) Serverless Cloud Computing Paradigm".to_string(),
            authors: "Lertpongrujikorn, Pawissanutt, and Mohsen Amini Salehi".to_string(),
            venue: "2024 IEEE 44th International Conference on Distributed Computing Systems Workshops (ICDCSW).".to_string(),
            pdf_link: "https://arxiv.org/pdf/2407.17391".to_string(),
            doi_link: "https://doi.org/10.1109/ICDCSW63686.2024.00006".to_string(),
        },
        Publication {
            title: "Object as a service (OaaS): Enabling object abstraction in serverless clouds".to_string(),
            authors: "Lertpongrujikorn, Pawissanutt, and Mohsen Amini Salehi".to_string(),
            venue: "2023 IEEE 16th International Conference on Cloud Computing (CLOUD).".to_string(),
            pdf_link: "https://arxiv.org/pdf/2206.05361".to_string(),
            doi_link: "https://doi.org/10.1109/CLOUD60044.2023.00035".to_string(),
        },
    ])
}

pub fn get_experiences() -> Signal<Vec<ExperienceItem>> {
    use_signal(|| vec![
        ExperienceItem {
            title: "Research Assistant".to_string(),
            company: "HPCC Lab".to_string(),
            dates: "Aug 2021 - Present".to_string(),
            description: "Pioneered the Object-as-a-Service (OaaS) paradigm and engineered \"Oparaca\" platforms using Java, Rust, and Python on Kubernetes/Knative.".to_string(),
            details: vec![
                ExperienceDetail {
                    label: None,
                    content: "For detailed information on Object-as-a-Service (OaaS) and Action Engine projects, please see the Featured Research section.".to_string(),
                    sub_details: vec![],
                    internal_link: Some("#research".to_string()),
                },
                ExperienceDetail {
                    label: None,
                    content: "Built automated scripts to streamline research testbed creation in Chameleon Cloud using Terraform and Ansible.".to_string(),
                    sub_details: vec![],
                    internal_link: None,
                },
                ExperienceDetail {
                    label: None,
                    content: "Mentored multiple Ph.D. students, providing technical guidance on distributed systems and research methodologies.".to_string(),
                    sub_details: vec![],
                    internal_link: None,
                },
            ],
            tags: vec!["OaaS".to_string(), "Java".to_string(), "Rust".to_string(), "Python".to_string(), "Kubernetes".to_string(), "Knative".to_string(), "Terraform".to_string(), "Ansible".to_string(), "LLMs".to_string(), "Distributed Systems".to_string()],
        },
        ExperienceItem {
            title: "Software Engineer".to_string(),
            company: "ByteArk".to_string(),
            dates: "Jun 2019 - Jul 2021".to_string(),
            description: "Engineered and launched a Low-Latency HLS (LL-HLS) service, reducing end-to-end latency to <2 seconds.".to_string(),
            details: vec![
                ExperienceDetail {
                    label: None,
                    content: "Engineered and launched a Low-Latency HLS (LL-HLS) service, reducing end-to-end latency to <2 seconds (screen-to-screen), enabling real-time interactivity for live events.".to_string(),
                    sub_details: vec![],
                    internal_link: None,
                },
                ExperienceDetail {
                    label: None,
                    content: "Engineered a distributed load testing framework for Pub/Sub systems, simulating over 50,000 concurrent virtual users to identify bottlenecks and ensure reliability during high-traffic live events.".to_string(),
                    sub_details: vec![],
                    internal_link: None,
                },
                ExperienceDetail {
                    label: None,
                    content: "Developed a Dynamic Ad Insertion (DAI) engine for HLS, implementing server-side manifest manipulation to enable seamless, buffer-free ad stitching for live streams.".to_string(),
                    sub_details: vec![],
                    internal_link: None,
                },
                ExperienceDetail {
                    label: None,
                    content: "Architected a dynamic priority queueing system for video transcoding, enabling seamless real-time priority switching to accommodate urgent customer requests, ensuring critical SLAs were met without pipeline disruption.".to_string(),
                    sub_details: vec![],
                    internal_link: None,
                },
                ExperienceDetail {
                    label: None,
                    content: "Modernized deployment infrastructure by implementing automated CI/CD pipelines using GitLab CI and Ansible, eliminating manual errors and ensuring consistent, reproducible releases across all environments.".to_string(),
                    sub_details: vec![],
                    internal_link: None,
                },
                ExperienceDetail {
                    label: None,
                    content: "Conducted in-depth research into video transcoding configurations to optimize transcoding time and streaming bandwidth.".to_string(),
                    sub_details: vec![],
                    internal_link: None,
                },
            ],
            tags: vec!["CDN".to_string(), "LL-HLS".to_string(), "GitLab CI".to_string(), "Ansible".to_string(), "Video Streaming".to_string(), "Performance Optimization".to_string()],
        },
        ExperienceItem {
            title: "Teaching Assistant".to_string(),
            company: "University of Louisiana at Lafayette".to_string(),
            dates: "Jan 2022 - May 2023".to_string(),
            description: "Assisted in teaching Distributed Computing Systems; guided students in setting up AWS cloud environments.".to_string(),
            details: vec![
                ExperienceDetail {
                    label: None,
                    content: "Assisted in teaching Distributed Computing Systems; guided students in setting up AWS cloud environments.".to_string(),
                    sub_details: vec![],
                    internal_link: None,
                },
            ],
            tags: vec!["Teaching".to_string(), "Distributed Computing".to_string(), "AWS".to_string()],
        },
    ])
}

pub fn get_education() -> Signal<Vec<EducationItem>> {
    use_signal(|| vec![
        EducationItem {
            degree: "Doctor of Philosophy - PhD, Computer Science and Engineering".to_string(),
            university: "University of North Texas, Denton, TX".to_string(),
            dates: "2021 - 2025".to_string(),
        },
        EducationItem {
            degree: "Master of Science - MS, Computer Science".to_string(),
            university: "University of North Texas, Denton, TX".to_string(),
            dates: "2021 - 2025".to_string(),
        },
        EducationItem {
            degree: "Bachelor of Engineering - BE, Computer Engineering".to_string(),
            university: "Kasetsart University, Bangkok, Thailand".to_string(),
            dates: "2015 - 2019".to_string(),
        },
    ])
}

pub fn get_skills() -> Signal<Vec<(&'static str, Vec<&'static str>)>> {
    use_signal(|| vec![
        ("Core Competencies", vec!["Distributed Systems", "Serverless Computing", "Cloud-Native Architecture", "Microservices", "System Design", "Performance Optimization", "Load Testing", "Video Streaming (FFmpeg, HLS, DASH)"]),
        ("Languages", vec!["Java", "Rust", "Python", "TypeScript", "SQL", "Bash"]),
        ("Cloud Native", vec!["Kubernetes", "Knative", "Docker", "AWS", "Linux"]),
        ("Databases & Storage", vec!["Redis", "MongoDB", "ArangoDB", "MinIO (S3)", "Infinispan"]),
        ("Messaging & Streaming", vec!["Kafka", "Zenoh", "MQTT"]),
        ("API & Protocols", vec!["gRPC", "Protobuf", "REST", "WebSocket"]),
        ("DevOps & IaC", vec!["Terraform", "Ansible", "GitLab CI", "GitHub Actions"]),
        ("Observability", vec!["Prometheus", "Grafana", "OpenTelemetry"]),
    ])
}
