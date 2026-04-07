Information technology — Artificial
intelligence — Management system
Technologies de l'information — Intelligence artificielle — Système de management
INTERNATIONAL
STANDARD
ISO/IEC
42001
First edition2023 -1 2
ISO/IEC 42001:2023(E)Reference number^
© ISO/IEC 2023
iTeh Standards
(https://standards.iteh.ai)
Document Preview
ISO/IEC 42001: 2023
https://standards.iteh.ai/catalog/standards/iso/bd05d78b-c39b-4578-b771-cf7c184d9410/iso-iec-42001- 2023

ii

COPYRIGHT PROTECTED DOCUMENT
© ISO/IEC 2023All rights reserved. Unless otherwise specified, or required in the context of its implementation, no part of this publication may
be reproduced or utilized otherwise in any form or by any means, electronic or mechanical, including photocopying, or posting on the internet or an intranet, without prior written permission. Permission can be requested from either ISO at the address below
or ISO’s member body in the country of the requester.ISO copyright office
CP 401 • Ch. de Blandonnet 8CH-1214 Vernier, Geneva
Phone: +41 22 749 01 11Email: copyright@iso.org

Published in SwitzerlandWebsite: http://www.iso.org
© ISO/IEC 2023 – All rights reserved

iTeh Standards
(https://standards.iteh.ai)
Document Preview
ISO/IEC 42001: 2023
https://standards.iteh.ai/catalog/standards/iso/bd05d78b-c39b-4578-b771-cf7c184d9410/iso-iec-42001- 2023
Foreword .......................................................................................................................................................................................................................................... v
© ISO/IEC 2023 – All rights reserved iii

Contents Page

iTeh Standards
(https://standards.iteh.ai)
1 Scope Introduction vi
2 Normative references
3 Terms and definitions
4 Context of the organization 4.1 Understanding the organization and its context
4.2 Understanding the needs and expectations of interested parties4.3 Determining the scope of the AI management system
4.4 AI management system
5 Leadership 5.1 Leadership and commitment
5.2 AI policy5.3 Roles, responsibilities and authorities
6 Planning 6.1 Actions to address risks and opportunities
- 6.1.1 General6.1.2 AI risk assessment
- 6.1.3 AI risk treatment6.1.4 AI system impact assessment
6.2 AI objectives and planning to achieve them6.3 Planning of changes
7 Support 7.1 Resources
7.2 Competence7.3 Awareness
7.4 Communication7.5 Documented information
7.5.1 General7.5.2 Creating and updating documented information
7.5.3 Control of documented information
8 Operation 8.1 Operational planning and control
8.2 AI risk assessment8.3 AI risk treatment
8.4 AI system impact assessment
9 Performance evaluation 9.1 Monitoring, measurement, analysis and evaluation
9.2 Internal audit9.2.1 General
9.3 Management review9.2.2 Internal audit programme
9.3.1 General9.3.2 Management review inputs
9.3.3 Management review results
10 Improvement 10.1 Continual improvement
10.2 Nonconformity and corrective action
Annex A (normative) Reference control objectives and controls
- ISO/IEC 42001: Document Preview
https://standards.iteh.ai/catalog/standards/iso/bd05d78b-c39b-4578-b771-cf7c184d9410/iso-iec-42001-
Annex B (normative) Implementation guidance for AI controls ....................................................................................... 21
Annex C (informative) Potential AI-related organizational objectives and risk sources ....................... 46
Annex D (informative) Use of the AI management system across domains or sectors ............................. 49
Bibliography ............................................................................................................................................................................................................................. 51

iv © ISO/IEC 2023 – All rights reserved

iTeh Standards
(https://standards.iteh.ai)
Document Preview
ISO/IEC 42001: 2023
https://standards.iteh.ai/catalog/standards/iso/bd05d78b-c39b-4578-b771-cf7c184d9410/iso-iec-42001- 2023
Foreword
ISO (the International Organization for Standardization) and IEC (the International Electrotechnical Commission) form the specialized system for worldwide standardization. National bodies that are
members of ISO or IEC participate in the development of International Standards through technical committees established by the respective organization to deal with particular fields of technical
activity. ISO and IEC technical committees collaborate in fields of mutual interest. Other international organizations, governmental and non-governmental, in liaison with ISO and IEC, also take part in the
work.
The procedures used to develop this document and those intended for its further maintenance are described in the ISO/IEC Directives, Part 1. In particular, the different approval criteria
needed for the different types of document should be noted. This document was drafted in accordance with the editorial rules of the ISO/IEC Directives, Part 2 (see http://www.iso.org/directives or
http://www.iec.ch/members_experts/refdocs).
ISO and IEC draw attention to the possibility that the implementation of this document may involve the use of (a) patent(s). ISO and IEC take no position concerning the evidence, validity or applicability of
any claimed patent rights in respect thereof. As of the date of publication of this document, ISO and IEC had not received notice of (a) patent(s) which may be required to implement this document. However,
implementers are cautioned that this may not represent the latest information, which may be obtained from the patent database available at http://www.iso.org/patents and https://patents.iec.ch. ISO and IEC shall
not be held responsible for identifying any or all such patent rights.
Any trade name used in this document is information given for the convenience of users and does not constitute an endorsement.
For an explanation of the voluntary nature of standards, the meaning of ISO specific terms and expressions related to conformity assessment, as well as information about ISO's adherence to
the World Trade Organization (WTO) principles in the Technical Barriers to Trade (TBT) see http://www.iso.org/iso/foreword.html. In the IEC, see http://www.iec.ch/understanding-standards.
This document was prepared by Joint Technical Committee ISO/IEC JTC Subcommittee SC 42, Artificial intelligence. 1, Information technology ,
Any feedback or questions on this document should be directed to the user’s national standards body. A complete listing of these bodies can be found at http://www.iso.org/members.html and
http://www.iec.ch/national-committees.
© ISO/IEC 2023 – All rights reserved v
iTeh Standards
(https://standards.iteh.ai)
Document Preview
ISO/IEC 42001: 2023
https://standards.iteh.ai/catalog/standards/iso/bd05d78b-c39b-4578-b771-cf7c184d9410/iso-iec-42001- 2023

Introduction

Artificial intelligence (AI) is increasingly applied across all sectors utilizing information technology and is expected to be one of the main economic drivers. A consequence of this trend is that certain
applications can give rise to societal challenges over the coming years.
This document intends to help organizations responsibly perform their role with respect to AI systems (e.g. to use, develop, monitor or provide products or services that utilize AI). AI potentially raises
specific considerations such as:
— The use of AI for automatic decision-making, sometimes in a non-transparent and non-explainable way, can require specific management beyond the management of classical IT systems.

— The use of data analysis, insight and machine learning, rather than human-coded logic to design systems, both increases the application opportunities for AI systems and changes the way that such
systems are developed, justified and deployed.
— AI systems that perform continuous learning change their behaviour during use. They require special consideration to ensure their responsible use continues with changing behaviour.

This document provides requirements for establishing, implementing, maintaining and continually improving an AI management system within the context of an organization. Organizations are expected
to focus their application of requirements on features that are unique to AI. Certain features of AI, such as the ability to continuously learn and improve or a lack of transparency or explainability, can warrant
different safeguards if they raise additional concerns compared to how the task would traditionally be performed. The adoption of an AI management system to extend the existing management structures is
a strategic decision for an organization.
The organization’s needs and objectives, processes, size and structure as well as the expectations of various interested parties influence the establishment and implementation of the AI management
system. Another set of factors that influence the establishment and implementation of the AI management system are the many use cases for AI and the need to strike the appropriate balance
between governance mechanisms and innovation. Organizations can elect to apply these requirements using a risk-based approach to ensure that the appropriate level of control is applied for the particular
AI use cases, services or products within the organization’s scope. All these influencing factors are expected to change and be reviewed from time to time.

The AI management system should be integrated with the organization’s processes and overall management structure. Specific issues related to AI should be considered in the design of processes,
information systems and controls. Crucial examples of such management processes are:
— determination of organizational objectives, involvement of interested parties and organizational policy;

— management of risks and opportunities;
— processes for the management of concerns related to the trustworthiness of AI systems such as security, safety, fairness, transparency, data quality and quality of AI systems throughout their life
cycle;
— processes for the management of suppliers, partners and third parties that provide or develop AI systems for the organization.

This document provides guidelines for the deployment of applicable controls to support such processes.
This document avoids specific guidance on management processes. The organization can combine generally accepted frameworks, other International Standards and its own experience to implement
crucial processes such as risk management, life cycle management and data quality management which are appropriate for the specific AI use cases, products or services within the scope.

vi © ISO/IEC 2023 – All rights reserved

iTeh Standards
(https://standards.iteh.ai)
Document Preview
ISO/IEC 42001: 2023
https://standards.iteh.ai/catalog/standards/iso/bd05d78b-c39b-4578-b771-cf7c184d9410/iso-iec-42001- 2023
An organization conforming with the requirements in this document can generate evidence of its responsibility and accountability regarding its role with respect to AI systems.
The order in which requirements are presented in this document does not reflect their importance or imply the order in which they are implemented. The list items are enumerated for reference purposes
only.
Compatibility with other management system standards
This document applies the harmonized structure (identical clause numbers, clause titles, text and common terms and core definitions) developed to enhance alignment among management system
standards (MSS). The AI management system provides requirements specific to managing the issues and risks arising from using AI in an organization. This common approach facilitates implementation
and consistency with other management system standards, e.g. related to quality, safety, security and privacy.
© ISO/IEC 2023 – All rights reserved vii
iTeh Standards
(https://standards.iteh.ai)
Document Preview
ISO/IEC 42001: 2023
https://standards.iteh.ai/catalog/standards/iso/bd05d78b-c39b-4578-b771-cf7c184d9410/iso-iec-42001- 2023

iTeh Standards
(https://standards.iteh.ai)
Document Preview
ISO/IEC 42001: 2023
https://standards.iteh.ai/catalog/standards/iso/bd05d78b-c39b-4578-b771-cf7c184d9410/iso-iec-42001- 2023

INTERNATIONAL STANDARD ISO/IEC 42001:2023(E)
Information technology — Artificial intelligence —
Management system
1 Scope
This document specifies the requirements and provides guidance for establishing, implementing, maintaining and continually improving an AI (artificial intelligence) management system within the
context of an organization.
This document is intended for use by an organization providing or using products or services that utilize AI systems. This document is intended to help the organization develop, provide or use AI
systems responsibly in pursuing its objectives and meet applicable requirements, obligations related to interested parties and expectations from them.
This document is applicable to any organization, regardless of size, type and nature, that provides or uses products or services that utilize AI systems.
2 Normative references
The following documents are referred to in the text in such a way that some or all of their content constitutes requirements of this document. For dated references, only the edition cited applies. For
undated references, the latest edition of the referenced document (including any amendments) applies.
ISO/IEC 22989:2022, and terminology Information technology — Artificial intelligence — Artificial intelligence concepts
3 Terms and definitions
For the purposes of this document, the terms and definitions given in ISO/IEC 22989 and the following apply.
ISO and IEC maintain terminology databases for use in standardization at the following addresses:
— ISO Online browsing platform: available at h t t p s : // www .iso .org/ obp
— IEC Electropedia: available at h t t p s : // www .electropedia .org/
3.1organization
person or group of people that has its own functions with responsibilities, authorities and relationships to achieve its objectives (3.6)
Note 1 to entry: The concept of organization includes, but is not limited to, sole-trader, company, corporation, firm, enterprise, authority, partnership, charity or institution or part or combination thereof, whether incorporated or
not, public or private.
Note 2 to entry: larger entity that is within the scope of the AI If the organization is part of a larger entity, the term “organization” refers only to the part of the management system (3.4).
3.2interested party
person or or activity organization (3.1) that can affect, be affected by, or perceive itself to be affected by a decision
Note 1 to entry: An overview of interested parties in AI is provided in ISO/IEC 22989:2022, 5.19.
© ISO/IEC 2023 – All rights reserved 1
iTeh Standards
(https://standards.iteh.ai)
Document Preview
ISO/IEC 42001: 2023
https://standards.iteh.ai/catalog/standards/iso/bd05d78b-c39b-4578-b771-cf7c184d9410/iso-iec-42001- 2023

3.3top management
person or group of people who directs and controls an organization (3.1) at the highest level
Note 1 to entry: organization. Top management has the power to delegate authority and provide resources within the

Note 2 to entry: management refers to those who direct and control that part of the organization.If the scope of the management system (3.4) covers only part of an organization, then top

3.4management system
set of interrelated or interacting elements of an objectives (3.6), as well as processes (3.8) to achieve those objectives organization (3.1) to establish policies (3.5) and

Note 1 to entry: A management system can address a single discipline or several disciplines.
Note 2 to entry: The management system elements include the organization’s structure, roles and responsibilities, planning and operation.

3.5policy
intentions and direction of an organization (3.1) as formally expressed by its top management (3.3)
3.6objective
result to be achieved
Note 1 to entry: An objective can be strategic, tactical, or operational.
Note 2 to entry: Objectives can relate to different disciplines (such as finance, health and safety, and environment). They can be, for example, organization-wide or specific to a project, product or process (3.8).

Note 3 to entry: operational criterion, as an AI objective or by the use of other words with similar meaning (e.g. aim, goal, or An objective can be expressed in other ways, e.g. as an intended result, as a purpose, as an
target).
Note 4 to entry: consistent with the AI In the context of AI policy (3.5), to achieve specific results. management systems (3.4), AI objectives are set by the organization (3.1),

3.7risk
effect of uncertainty
Note 1 to entry: An effect is a deviation from the expected — positive or negative.
Note 2 to entry: knowledge of, an event, its consequence, or likelihood.Uncertainty is the state, even partial, of deficiency of information related to, understanding or

Note 3 to entry: consequences (as defined in ISO Guide 73), or a combination of these.Risk is often characterized by reference to potential events (as defined in ISO Guide 73) and

Note 4 to entry: changes in circumstances) and the associated likelihood (as defined in ISO Guide 73) of occurrence.Risk is often expressed in terms of a combination of the consequences of an event (including

3.8process
set of interrelated or interacting activities that uses or transforms inputs to deliver a result
Note 1 to entry: Whether the result of a process is called an output, a product or a service depends on the context of the reference.

2 © ISO/IEC 2023 – All rights reserved

iTeh Standards
(https://standards.iteh.ai)
Document Preview
ISO/IEC 42001: 2023
https://standards.iteh.ai/catalog/standards/iso/bd05d78b-c39b-4578-b771-cf7c184d9410/iso-iec-42001- 2023
3.9competence
ability to apply knowledge and skills to achieve intended results
3.10documented information
information required to be controlled and maintained by an which it is contained organization (3.1) and the medium on
Note 1 to entry: Documented information can be in any format and media and from any source.
Note 2 to entry: Documented information can refer to:
— the management system (3.4), including related processes (3.8);
— information created in order for the organization to operate (documentation);
— evidence of results achieved (records).
3.11performance
measurable result
Note 1 to entry: Performance can relate either to quantitative or qualitative findings.
Note 2 to entry: organizations (3.1Performance can relate to managing activities, ). processes (3.8), products, services, systems or
Note 3 to entry: In the context of this document, performance refers both to results achieved by using AI systems and results related to the AI management system (3.4). The correct interpretation of the term is clear from the
context of its use.
3.12continual improvement
recurring activity to enhance performance (3.11)
3.13effectiveness
extent to which planned activities are realized and planned results are achieved
3.14requirement
need or expectation that is stated, generally implied or obligatory
Note 1 to entry: interested parties “Generally implied” means that it is custom or common practice for the (3.2) that the need or expectation under consideration is implied. organization (3.1) and
Note 2 to entry: A specified requirement is one that is stated, e.g. in documented information (3.10).
3.15conformity
fulfilment of a requirement (3 .14)
3.16nonconformity
non-fulfilment of a requirement (3 .14)
3.17corrective action
action to eliminate the cause(s) of a nonconformity (3.16) and to prevent recurrence
© ISO/IEC 2023 – All rights reserved 3
iTeh Standards
(https://standards.iteh.ai)
Document Preview
ISO/IEC 42001: 2023
https://standards.iteh.ai/catalog/standards/iso/bd05d78b-c39b-4578-b771-cf7c184d9410/iso-iec-42001- 2023

3.18audit
systematic and independent determine the extent to which the audit criteria are fulfilled process (3.8) for obtaining evidence and evaluating it objectively to

Note 1 to entry: and it can be a combined audit (combining two or more disciplines).An audit can be an internal audit (first party) or an external audit (second party or third party),

Note 2 to entry: behalf. An internal audit is conducted by the organization (3.1) itself, or by an external party on its

Note 3 to entry: “Audit evidence” and “audit criteria” are defined in ISO 19011.
3.19measurement
process (3.8) to determine a value
3.20monitoring
determining the status of a system, a process (3.8) or an activity
Note 1 to entry: To determine the status, there can be a need to check, supervise or critically observe.
3.21control
measure that maintains and/or modifies risk (3.7)
Note 1 to entry: and/or actions which maintain and/or modify risk.Controls include, but are not limited to, any process, policy, device, practice or other conditions

Note 2 to entry: Controls may not always exert the intended or assumed modifying effect.
[SOURCE: ISO 31000:2018, 3.8, modified — Added as application domain ]
3.22governing body
person or group of people who are accountable for the performance and conformance of the organization
Note 1 to entry: Not all organizations, particularly small organizations, will have a governing body separate from top management.

Note 2 to entry:supervisory board, trustees or overseers. A governing body can include, but is not limited to, board of directors, committees of the board,

[SOURCE: ISO/IEC 38500:2015, 2.9, modified — Added Notes to entry.]
3.23information security
preservation of confidentiality, integrity and availability of information
Note 1 to entry: Other properties such as authenticity, accountability, non-repudiation and reliability can also be involved.

[SOURCE: ISO/IEC 27000:2018, 3.28]
3.24AI system impact assessment
formal, documented process by which the impacts on individuals, groups of individuals, or both, and societies are identified, evaluated and addressed by an organization developing, providing or using
products or services utilizing artificial intelligence

4 © ISO/IEC 2023 – All rights reserved

iTeh Standards
(https://standards.iteh.ai)
Document Preview
ISO/IEC 42001: 2023
https://standards.iteh.ai/catalog/standards/iso/bd05d78b-c39b-4578-b771-cf7c184d9410/iso-iec-42001- 2023
3.25data quality
characteristic of data that the data meet the organization’s data requirements for a specific context
[SOURCE: ISO/IEC 5259-1:—1), 3.4]
3.26statement of applicability
documentation of all necessary controls (3.23) and justification for inclusion or exclusion of controls
Note 1 to entry: Annex A with additional controls established by the organization itself.Organizations may not require all controls listed in Annex A or may even exceed the list in
Note 2 to entry: this document. All identified risks and the risk management measures (controls) established to address them All identified risks shall be documented by the organization according to the requirements of
shall be reflected in the statement of applicability.
4 Context of the organization
4.1 Understanding the organization and its context
The organization shall determine external and internal issues that are relevant to its purpose and that affect its ability to achieve the intended result(s) of its AI management system.
The organization shall determine whether climate change is a relevant issue.
The organization shall consider the intended purpose of the AI systems that are developed, provided or used by the organization. The organization shall determine its roles with respect to these AI systems.
NOTE 1 role relative to the AI system. These roles can include, but are not limited to, one or more of the following:To understand the organization and its context, it can be helpful for the organization to determine its
— AI providers, including AI platform providers, AI product or service providers;
— AI producers, including AI developers, AI designers, AI operators, AI testers and evaluators, AI deployers, AI human factor professionals, domain experts, AI impact assessors, procurers, AI governance and oversight
professionals;
— AI customers, including AI users;
— AI partners, including AI system integrators and data providers;
— AI subjects, including data subjects and other subjects;
— relevant authorities, including policymakers and regulators.
A detailed description of these roles is provided by ISO/IEC 22989. Furthermore, the types of roles and their relationship to the AI system life cycle are also described in the NIST AI risk management framework.[ 29 ] The
organization’s roles can determine the applicability and extent of applicability of the requirements and controls in this document.
NOTE 2 roles and jurisdiction and their impact on its ability to achieve the intended outcome(s) of its AI management External and internal issues to be addressed under this clause can vary according to the organization’s
system. These can include, but are not limited to:
a) external context related considerations such as:
1) applicable legal requirements, including prohibited uses of AI;
2) policies, guidelines and decisions from regulators that have an impact on the interpretation or enforcement of legal requirements in the development and use of AI systems;
1) Under preparation. Stage at the time of publication ISO/IEC DIS 5259-1:2023.
© ISO/IEC 2023 – All rights reserved 5
iTeh Standards
(https://standards.iteh.ai)
Document Preview
ISO/IEC 42001: 2023
https://standards.iteh.ai/catalog/standards/iso/bd05d78b-c39b-4578-b771-cf7c184d9410/iso-iec-42001- 2023

incentives or consequences associated with the intended purpose and the use of AI systems;
culture, traditions, values, norms and ethics with respect to development and use of AI;
competitive landscape and trends for new products and services using AI systems;
b) internal context related considerations such as:
organizational context, governance, objectives (see 6.2), policies and procedures;
contractual obligations;
intended purpose of the AI system to be developed or used.
NOTE 3 processes (e.g. personally identifiable information (PII) processor or PII controller when processing PII). See Role determination can be formed by obligations related to categories of data the organization
ISO/IEC 29100 for PII and related roles. Roles can also be informed by legal requirements specific to AI systems.
4.2 Understanding the needs and expectations of interested parties
The organization shall determine:
— the interested parties that are relevant to the AI management system;
— the relevant requirements of these interested parties;
— which of these requirements will be addressed through the AI management system.
NOTE Relevant interested parties can have requirements related to climate change.
4.3 Determining the scope of the AI management system
The organization shall determine the boundaries and applicability of the AI management system to establish its scope.
When determining this scope, the organization shall consider:
— the external and internal issues referred to in 4.1;
— the requirements referred to in 4.2.
The scope shall be available as documented information.
The scope of the AI management system shall determine the organization’s activities with respect to this document’s requirements on the AI management system, leadership, planning, support, operation,
performance, evaluation, improvement, controls and objectives.

4.4 AI management system
The organization shall establish, implement, maintain, continually improve and document an AI management system, including the processes needed and their interactions, in accordance with the
requirements of this document.

6 © ISO/IEC 2023 – All rights reserved

iTeh Standards
(https://standards.iteh.ai)
Document Preview
ISO/IEC 42001: 2023
https://standards.iteh.ai/catalog/standards/iso/bd05d78b-c39b-4578-b771-cf7c184d9410/iso-iec-42001- 2023