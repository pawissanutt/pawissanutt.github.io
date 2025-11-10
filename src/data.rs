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
pub struct ExperienceItem {
    pub title: String,
    pub company: String,
    pub dates: String,
    pub description: String,
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
            description: "Novel serverless paradigm that encapsulates application data, business logic, and non-functional requirements (QoS, constraints) into unified deployment packages, eliminating provider lock-in and manual data management overhead inherent in traditional FaaS.".to_string(),
            highlights: vec![
                "Implements OOP principles (inheritance, access modifiers) and dataflow semantics with transparent parallelism, synchronization, and fault-tolerance in serverless environments.".to_string(),
                "Enables declarative QoS enforcement while significantly reducing development time and complexity compared to traditional FaaS approaches.".to_string(),
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
            description: "Tool-augmented LLM system that interprets natural language queries and automatically generates platform-agnostic FaaS workflows, eliminating the need for specialized cloud expertise and reducing vendor lock-in.".to_string(),
            highlights: vec![
                "Automates function identification, data dependency management, and workflow execution from human language queries.".to_string(),
                "Maintains language and platform independence across AWS, Google Cloud, and other cloud providers.".to_string(),
            ],
            github_link: "https://github.com/hpcclab/action_engine".to_string(),
            publications: vec![
                ResearchPublication {
                    title: "Future Generation Computer Systems (2026)".to_string(),
                    link: "https://arxiv.org/pdf/2411.19485".to_string(),
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
            description: "Pioneered the design and development of Object-as-a-Service (OaaS). Engineered and implemented the OaaS prototype using Java, Rust, and Python, leveraging Kubernetes and Knative for robust deployment. Conducted rigorous performance evaluations on a distributed serverless system, using comprehensive load testing to diagnose and resolve complex technical bottlenecks, directly improving system scalability and reliability.".to_string(),
            tags: vec!["OaaS".to_string(), "Java".to_string(), "Rust".to_string(), "Python".to_string(), "Kubernetes".to_string(), "Knative".to_string(), "Terraform".to_string(), "Ansible".to_string(), "LLMs".to_string(), "Distributed Systems".to_string()],
        },
        // ExperienceItem {
        //     title: "Project: Object-as-a-Service (OaaS) (NSF-Funded)".to_string(),
        //     company: "University of North Texas".to_string(),
        //     dates: "Aug 2021 - Present".to_string(),
        //     description: "Developed a novel serverless paradigm to address the limitations of FaaS. Streamlined cloud-native application development by encapsulating business logic, data, and non-functional requirements into a single, cohesive deployment package. Integrated with Kubernetes, Kafka, Zenoh, and Minio (S3).".to_string(),
        //     tags: vec!["Serverless".to_string(), "Kubernetes".to_string(), "Kafka".to_string(), "Zenoh".to_string(), "Minio".to_string(), "IoT".to_string()],
        // },
        // ExperienceItem {
        //     title: "Project: Action Engine".to_string(),
        //     company: "University of North Texas".to_string(),
        //     dates: "Jan 2023 - May 2025".to_string(),
        //     description: "Developed a developer tool that automatically generates serverless workflows from natural language descriptions using Large Language Models (LLMs). Engineered a platform-agnostic system that outputs workflow definitions compatible with various cloud providers, reducing vendor lock-in.".to_string(),
        //     tags: vec!["LLMs".to_string(), "FaaS".to_string(), "Serverless".to_string(), "Platform-Agnostic".to_string(), "AWS Step Functions".to_string(), "Google Cloud Composer".to_string()],
        // },
        ExperienceItem {
            title: "Teaching Assistant".to_string(),
            company: "University of Louisiana at Lafayette".to_string(),
            dates: "Jan 2022 - May 2023".to_string(),
            description: "Assisted in teaching the Distributed Computing Systems class (CSCE 533). Graded assignments and exams, providing constructive feedback, and helped students set up environments on the AWS cloud.".to_string(),
            tags: vec!["Teaching".to_string(), "Distributed Computing".to_string(), "AWS".to_string()],
        },
        ExperienceItem {
            title: "Software Engineer".to_string(),
            company: "ByteArk".to_string(),
            dates: "Jun 2019 - Jul 2021".to_string(),
            description: "Developed software for content delivery networks (CDN) and video streaming. Engineered and launched a low-latency HTTP live streaming (LL-HLS) service. Optimized software performance through extensive load testing, which improved reliability and performance for high-traffic video streaming.".to_string(),
            tags: vec!["CDN".to_string(), "LL-HLS".to_string(), "GitLab CI".to_string(), "Ansible".to_string(), "Video Streaming".to_string(), "Performance Optimization".to_string()],
        },
    ])
}

pub fn get_education() -> Signal<Vec<EducationItem>> {
    use_signal(|| vec![
        EducationItem {
            degree: "Doctor of Philosophy - PhD, Computer Science and Engineering".to_string(),
            university: "University of North Texas, Denton, Texas, USA".to_string(),
            dates: "2021 - Present".to_string(),
        },
        EducationItem {
            degree: "Master of Science - MS, Computer Science".to_string(),
            university: "University of North Texas, Denton, Texas, USA".to_string(),
            dates: "2021 - 2025".to_string(), // Dates not specified in resume for MS
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
        ("Technical", vec!["Cloud Computing", "Microservices", "Distributed Systems", "System Architecture", "Load Testing", "Performance Optimization", "Object-Oriented Programming", "Algorithm"]),
        ("Programming Language", vec!["Java", "Rust", "Python", "TypeScript"]),
        ("Cloud & DevOps", vec!["AWS", "Chameleon Cloud", "Kubernetes", "Knative", "Docker", "Terraform", "Ansible", "GitLab CI", "GitHub Action"]),
        ("Database & Storage", vec!["MongoDB", "ArangoDB", "Redis", "Minio (S3)", "Infinispan", "SQL"]),
        ("Messaging", vec!["Kafka", "Zenoh", "MQTT"]),
    ])
}
