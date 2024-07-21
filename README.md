# Projeto de Implementação de Plataforma Unificada do CREA

## 1. Introdução

Este documento descreve o plano para a implementação de uma plataforma unificada para registro e gestão de profissionais do Sistema Confea/Crea. O objetivo é criar um sistema centralizado que facilite o acesso dos profissionais a seus registros e permita a atuação em todo o território nacional de forma simplificada, especialmente em áreas interdisciplinares ou fora da área específica de graduação.

## 2. Objetivos

- Unificar o registro dos profissionais do CREA em uma plataforma integrada e acessível.
- Implementar um sistema flexível que reconheça e valide competências interdisciplinares.
- Reduzir a burocracia para permitir que profissionais atuem em diferentes estados sem necessidade de registros adicionais.
- Integrar a emissão de Anotações de Responsabilidade Técnica (ARTs) padronizadas e compatíveis com as competências interseccionais.
- Garantir segurança, interoperabilidade e conformidade com a LGPD.

## 3. Abordagem Proposta

Para atingir os objetivos mencionados, o projeto será dividido nas seguintes fases:

### 3.1. Desenvolvimento da Plataforma

É necessário uma plataforma centralizada baseada em tecnologias modernas para garantir eficiência e escalabilidade. As ferramentas sugeridas para o desenvolvimento são:

- **Frontend:** React.js para a interface de usuário intuitiva e responsiva.
- **Backend:** Rust para a lógica de negócios e integração de sistemas.
- **Banco de Dados:** PostgreSQL para armazenamento seguro e confiável dos registros.
- **Autenticação:** Implementação de OAuth 2.0 para controle de acesso seguro.
- **DevOps:** Utilização de Docker para facilitar a implantação e escalabilidade do sistema (ou Amazon AWS se os custos permitirem).
- **Segurança:** Uso de SSL/TLS para proteger os dados sensíveis dos profissionais.

### 3.2. Unificação das Interseções de Competências do CREA

Atualmente, as "pontes" entre diferentes áreas de atuação no CREA são geralmente tratadas através da emissão de ARTs (Anotações de Responsabilidade Técnica) específicas para cada atividade realizada. Essas ARTs são emitidas pelos profissionais do CREA responsáveis pela execução de serviços técnicos, e cada profissional deve possuir registro específico na área correspondente à atividade que está sendo realizada.

Para profissionais que desejam trabalhar fora da área de sua graduação em diferentes estados, a abordagem atual envolve solicitar o registro específico (ou "visto") no CREA da região onde o serviço será realizado. Isso é necessário porque cada CREA regional possui suas próprias regulamentações e normativas específicas, e o profissional deve estar registrado na área correspondente à atividade técnica que será exercida naquela região.

Para unificar e facilitar a atuação de profissionais em áreas interdisciplinares ou fora de suas áreas de graduação, é possível implementar um sistema que reconheça e valide as competências e habilidades que são comuns entre diferentes áreas profissionais. 

### 3.2.1. Reconhecimento de Competências Interdisciplinares

Identificação e certificação das competências interdisciplinares através de um sistema integrado de validação. Isso permitirá que profissionais com competências em áreas interseccionais (mesma competência, mas graduações diferentes) obtenham uma certificação que os habilite a exercer atividades nessas áreas sem necessidade de múltiplos registros.

**Campos Obrigatórios para Anotação de Responsabilidade Técnica (ART)**

Para garantir consistência e conformidade nas Anotações de Responsabilidade Técnica (ARTs) emitidas pelos profissionais registrados, os seguintes campos são considerados essenciais:

1. **Dados do Profissional**
   - Nome completo
   - Número de registro no CREA
   - CPF
   - Endereço completo
   - Telefone de contato
   - E-mail

2. **Dados do Contratante**
   - Nome/Razão Social
   - CPF/CNPJ
   - Endereço completo
   - Telefone de contato
   - E-mail (se aplicável)

3. **Descrição do Serviço**
   - Descrição detalhada do serviço técnico a ser realizado
   - Especificações técnicas relevantes

4. **Local da Execução do Serviço**
   - Endereço completo onde o serviço será executado

5. **Data de Início e Término do Serviço**
   - Data prevista para início e término do serviço técnico

6. **Assinatura e Responsabilidade**
   - Assinatura digital ou eletrônica do profissional responsável pela execução do serviço

7. **Registro da ART**
   - Número da ART gerado pelo sistema
   - Data de emissão da ART

**Formulário Principal da ART**

O formulário principal conterá os campos essenciais comuns a todas as profissões registradas no CREA. Esses campos serão obrigatórios para todas as ARTs emitidas e garantirão a uniformidade e consistência nas informações fornecidas.

**Formulário Específico por Área Profissional**

Além do formulário principal, será criado um conjunto de formulários específicos por área profissional (Engenharia Civil, Engenharia Ambiental, Agronomia, etc.). Cada formulário específico conterá campos adicionais que são específicos para a área de atuação do profissional.

### 3.3. Registro Flexível e Unificado

Implementação de um sistema de registro flexível onde profissionais podem registrar-se em áreas interdisciplinares reconhecidas, além de suas áreas específicas de formação. Isso reduzirá a necessidade de registros adicionais para atividades técnicas interseccionais.

### 3.4. Emissão de ARTs Padronizadas e Compatíveis

Integração da funcionalidade de emissão de ARTs no sistema centralizado, com modelos padronizados que levam em consideração as competências interseccionais dos profissionais. Isso garantirá que as ARTs emitidas sejam compatíveis com as atividades técnicas realizadas, independentemente da área específica de formação do profissional.

### 3.5. Integração e Interoperabilidade

Todos os CREAs regionais serão integrados à plataforma centralizada, garantindo que os dados sejam consistentes e facilmente acessíveis em todo o Brasil. Serão adotados padrões de interoperabilidade para compartilhamento seguro de informações entre sistemas regionais e a plataforma nacional.

### 3.6. Treinamento e Suporte

Desenvolvimento de materiais de treinamento e suporte técnico para todos os usuários da plataforma, incluindo profissionais e administradores regionais. Isso assegurará que todos estejam capacitados para utilizar efetivamente o sistema unificado e seus recursos.

## 4. Considerações Finais

Este projeto visa não apenas unificar e modernizar o registro dos profissionais do Sistema Confea/Crea e Mútua, mas também facilitar a atuação em áreas interdisciplinares e fora da área de formação específica. A implementação de um sistema flexível de registro e a emissão de ARTs compatíveis com competências interseccionais promoverá maior eficiência e mobilidade profissional, reduzindo a burocracia e promovendo a integração nacional.

## 5. Próximos Passos

Os próximos passos incluem a fase de desenvolvimento inicial da plataforma, seguida de testes rigorosos, integração com sistemas existentes e lançamento gradual em todas as regiões do Brasil. Um cronograma detalhado será elaborado para orientar a implementação, garantindo que todas as etapas sejam concluídas dentro dos prazos estabelecidos.

---

Licença MIT