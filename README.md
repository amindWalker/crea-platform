<div align="center">

# Plataforma Unificada do CREA

### [Visão Geral do Produto](https://github.com/amindWalker/crea-platform/blob/main/PRODUTO.md)

</div>

# 1. Introdução

Este documento descreve o plano para a implementação de uma plataforma unificada para registro e gestão de profissionais do Sistema Confea/Crea. O objetivo é criar um sistema centralizado que facilite o acesso dos profissionais a seus registros e permita a atuação em todo o território nacional de forma simplificada, especialmente em áreas interdisciplinares ou fora da área específica de graduação.

# 2. Objetivos

- Unificar o registro dos profissionais do CREA em uma plataforma integrada e acessível.
- Implementar um sistema flexível que reconheça e valide competências interdisciplinares.
- Reduzir a burocracia para permitir que profissionais atuem em diferentes estados sem necessidade de registros adicionais.
- Integrar a emissão de Anotações de Responsabilidade Técnica (ARTs) padronizadas e compatíveis com as competências interseccionais.
- Garantir segurança, interoperabilidade e conformidade com a Lei Geral de Proteção de Dados Pessoais (LGPD).

# 3. Abordagem Proposta

Para atingir os objetivos mencionados, o projeto será dividido nas seguintes fases:

## 3.1. Desenvolvimento da Plataforma

É necessário uma plataforma centralizada baseada em tecnologias modernas para garantir eficiência e escalabilidade. As ferramentas sugeridas para o desenvolvimento são:

- **Frontend:** React.js para a interface de usuário intuitiva e responsiva.
- **Backend:** Rust para a lógica de negócios e integração de sistemas.
- **Banco de Dados:** PostgreSQL para armazenamento seguro e confiável dos registros.
- **Autenticação:** Implementação de OAuth 2.0 para controle de acesso seguro.
- **DevOps:** Utilização de Docker para facilitar a implantação e escalabilidade do sistema (ou Amazon AWS se os custos permitirem).
- **Segurança:** Uso de SSL/TLS para proteger os dados sensíveis dos profissionais.

## 3.2. Unificação das Interseções de Competências do CREA

> [!IMPORTANT]
> Sistema de Reconhecimento de Competências
>
> #### Desafio Atual
>
> Atualmente, as competências interdisciplinares no CREA são tratadas por meio de Anotações de Responsabilidade Técnica (ARTs) específicas para cada atividade, emitidas regionalmente, o famoso visto. Isso exige que profissionais obtenham registros específicos em cada estado onde desejam atuar fora de suas áreas de graduação. Cada CREA regional tem suas próprias normativas, o que torna o processo ainda mais complexo e demorado.

#### Nossa Proposta

Para resolver esses desafios, propomos a implementação de um sistema integrado de reconhecimento e validação de competências interdisciplinares baseado na nova TOS Nacional (Tabela de Obras e Serviços Nacional). Esse sistema permitirá que profissionais com habilidades em áreas interseccionais recebam uma certificação única, reconhecida nacionalmente pelo CREA. Isso significa que poderão exercer atividades nessas áreas sem a necessidade de múltiplos registros.

### Benefícios

- **Redução de Burocracia:** Simplificação do processo para profissionais que desejam trabalhar em diferentes estados.
- **Eficiência Operacional:** Integração de competências em uma plataforma centralizada, promovendo maior eficiência e agilidade.
- **Flexibilidade Profissional:** Permitir que engenheiros e agrônomos explorem novas áreas de atuação sem barreiras regulatórias excessivas.

### Campos Obrigatórios para Anotação de Responsabilidade Técnica (ART)

Para garantir consistência e conformidade nas Anotações de Responsabilidade Técnica (ARTs) emitidas pelos profissionais registrados, os seguintes campos serão propostos:

<table width="100%">
  <tr>
    <td valign="top" width="50%">

1. **Exemplo de Formulário Principal da ART**
   
    - **Dados do Profissional**
      - Nome completo
      - Número de registro no CREA
      - CPF
      - Endereço completo
      - Telefone de contato
      - E-mail

    - **Dados do Contratante**
      - Nome/Razão Social
      - CPF/CNPJ
      - Endereço completo
      - Telefone de contato
      - E-mail (se aplicável)

    - **Descrição do Serviço**
      - Descrição detalhada do serviço técnico a ser realizado
      - Especificações técnicas relevantes

    - **Local da Execução do Serviço**
      - Endereço completo onde o serviço será executado

    - **Data de Início e Término do Serviço**
      - Data prevista para início e término do serviço técnico

    - **Assinatura e Responsabilidade**
      - Assinatura digital ou eletrônica do profissional responsável pela execução do serviço

    - **Registro da ART**
      - Número da ART gerado pelo sistema
      - Data de emissão da ART

  </td>

  <td valign="top" width="50%">

2. **Exemplo de Formulário Específico por Área Profissional**
    
    **Área Profissional - Engenharia Civil**

    **1. Dados do Projeto**

    - Tipo de obra: (ex: Edificação, Ponte, Estrada)
    - Descrição detalhada do projeto:
      - Incluir especificações técnicas como resistência do concreto, tipo de fundação, etc.

    **2. Materiais Utilizados**

    - Listagem dos materiais principais:
      - Quantidade, tipo, e normas técnicas aplicáveis.

    **3. Normas e Regulamentações Locais**

    - Normas específicas da região onde o projeto será executado:
      - Exigências municipais ou estaduais que devem ser seguidas.

    **4. Responsabilidades e Assinatura**

    - Nome do engenheiro responsável pelo projeto:
      - Assinatura digital ou eletrônica.

    **Área Profissional - Engenharia Elétrica**

    **1. Especificações Elétricas**

    - Potência requerida:
      - Detalhes dos circuitos elétricos e equipamentos.

    **2. Segurança Elétrica**

    - Medidas de segurança adotadas:
      - Normas de proteção contra choque elétrico, SPDA, etc.

    **3. Compatibilidade eletromagnética**

    - Garantia de conformidade com normas de compatibilidade:
      - Detalhes sobre interferências eletromagnéticas.

    **4. Certificações e Documentação**

    - Documentação necessária para aprovação:
      - Certificados de conformidade, relatórios técnicos, etc.

    **Área Profissional - Engenharia Ambiental**

    **1. Avaliação Ambiental Preliminar**

    - Impactos ambientais esperados:
      - Estratégias de mitigação e compensação.

    **2. Legislação Ambiental Aplicável**

    - Normas ambientais locais:
      - Restrições e licenças necessárias.

    **3. Monitoramento Ambiental**

    - Plano de monitoramento durante a execução:
      - Métodos de amostragem, parâmetros monitorados.

    **4. Certificação Ambiental**

    - Certificados necessários para conformidade:
      - Declarações de impacto ambiental, relatórios de auditoria.

    
  </td>
  </tr>
</table>

> [!NOTE]
> Além do formulário principal com os dados obrigatórios, serão criados campos específicos nos formulários para cada área profissional (Engenharia Civil, Engenharia Elétrica, Engenharia Ambiental, etc.). Os detalhes dos campos específicos serão implementados de forma a garantir consistência e permitir que diferentes formações possam exercer uma mesma função caso o curso possua competências compatíveis.
> ### Observações:
>
> - Cada formulário específico será adaptado às necessidades da respectiva área profissional, incorporando campos que são cruciais para a execução e documentação de projetos específicos.
> - Esses formulários complementam o formulário principal da ART, garantindo que cada área profissional tenha a capacidade de detalhar informações técnicas relevantes para suas atividades.
> - A implementação desses formulários visa permitir que profissionais de diferentes formações possam exercer funções além de suas especializações primárias, desde que sejam qualificados e certificados conforme as normas do sistema CONFEA/CREA.

## Exemplos na Prática:

  ### Exemplo 1: Coordenação de Projetos de Infraestrutura Elétrica

  - **Descrição:** Um engenheiro civil coordena a execução de projetos de infraestrutura elétrica em construções complexas.

    **Equipe:**
    - **Engenheiro civil:** Planeja e integra os sistemas de engenharia.
    - **Engenheiro elétrico:** Desenvolve os projetos elétricos.
    - **Engenheiro de segurança:** Garante a conformidade com normas de segurança.

  ### Exemplo 2: Projetos Sustentáveis de Edificações

  - **Descrição:** Um engenheiro ambiental especializado em impacto ambiental colabora com engenheiros civis para desenvolver edificações sustentáveis.

    **Equipe:**
    - **Engenheiro ambiental:** Realiza estudos e propõe medidas ambientais.
    - **Engenheiro civil:** Incorpora soluções sustentáveis no projeto.
    - **Arquiteto:** Projeta com foco em eficiência energética e sustentabilidade.

  ### Exemplo 3: Gestão de Processos Industriais em Indústria Química

  - **Descrição:** Um engenheiro de produção gerencia processos industriais em uma indústria química, aplicando normas de qualidade e segurança.

    **Equipe:**
    - **Engenheiro de produção:** Gerencia a produção e qualidade.
    - **Engenheiro químico:** Supervisiona processos conforme normativas.
    - **Engenheiro de segurança:** Avalia riscos e implementa medidas de segurança.

  ### Exemplo 4: Recuperação de Margens de Rios

  - **Descrição:** Recuperação de áreas afetadas por enchentes, estabilizando margens e restaurando a biodiversidade.

    **Equipe:**
    - **Engenheiro civil:** Lidera a estabilização das margens e obras de contenção.
    - **Engenheiro ambiental:** Avalia impactos e propõe medidas sustentáveis.
    - **Agrônomo:** Implementa práticas de manejo e plantio para proteção do solo.

  ### Exemplo 5: Reconstrução de Infraestrutura Urbana

  - **Descrição:** Coordenação da reconstrução urbana após desastres naturais, focando em resiliência e sustentabilidade.

    **Equipe:**
    - **Engenheiro civil:** Reconstrói pontes e estradas para resiliência a enchentes.
    - **Engenheiro ambiental:** Melhora gestão da água e sistemas pluviais urbanos.
    - **Agrônomo:** Aplica técnicas de manejo para melhorar a permeabilidade do solo.

## 3.3. Registro Flexível e Unificado

Implementação de um sistema de registro flexível onde profissionais podem registrar-se em áreas interdisciplinares reconhecidas, além de suas áreas específicas de formação. Isso reduzirá a necessidade de registros adicionais para atividades técnicas interseccionais.

## 3.4. Emissão de ARTs Padronizadas e Compatíveis

Integração da funcionalidade de emissão de ARTs no sistema centralizado, com modelos padronizados que levam em consideração as competências interseccionais dos profissionais. Isso garantirá que as ARTs emitidas sejam compatíveis com as atividades técnicas realizadas, independentemente da área específica de formação do profissional.

## 3.5. Integração e Interoperabilidade

Todos os CREAs regionais serão integrados à plataforma centralizada, garantindo que os dados sejam consistentes e facilmente acessíveis em todo o Brasil. Serão adotados padrões de interoperabilidade para compartilhamento seguro de informações entre sistemas regionais e a plataforma nacional.

## 3.6. Treinamento e Suporte

Desenvolvimento de materiais de treinamento e suporte técnico para todos os usuários da plataforma, incluindo profissionais e administradores regionais. Isso assegurará que todos estejam capacitados para utilizar efetivamente o sistema unificado e seus recursos.

# 4. Valor Agregado

Com nossa plataforma, sua operação se torna mais eficiente e colaborativa, garantindo resultados excelentes com menos esforço.

| Vantagem                         | Descrição                                                               |
| -------------------------------- | ----------------------------------------------------------------------- |
| Agilidade Ampliada               | Reduzimos o tempo para tarefas complexas, permitindo foco no essencial. |
| Comodidade                       | Simplificamos o processo para uma experiência eficiente.                |
| Menos Burocracia                 | Eliminamos obstáculos com procedimentos diretos.                        |
| Processos Ágeis e Simplificados  | Reduzimos etapas e economizamos recursos.                               |
| Plataforma Intuitiva             | Interface intuitiva para fácil navegação.                               |
| Redução de Erros Humanos         | Automação das etapas diminuem as margens de erro.                       |
| Maior Integração entre CREAS     | Colaboração facilitada entre CREAS para melhor atendimento.             |
| Facilidade de Acesso a Cadastros | Gerenciamento simplificado de cadastros.                                |


# 5. Considerações Finais

Este projeto visa não apenas unificar e modernizar o registro dos profissionais do Sistema Confea/Crea, mas também facilitar a atuação em áreas interdisciplinares e fora da área de formação específica. A implementação de um sistema flexível de registro e a emissão de ARTs compatíveis com competências interseccionais promoverá maior eficiência e mobilidade profissional, reduzindo a burocracia e promovendo a integração nacional.

# 6. Próximos Passos

Os próximos passos incluem a fase de desenvolvimento inicial da plataforma, seguida de testes rigorosos e lançamento gradual em todas as regiões do Brasil. Um cronograma detalhado será elaborado para orientar a implementação, garantindo que todas as etapas sejam concluídas dentro dos prazos estabelecidos.

# 7. Utilidades Extras (opcional)

Profissionais do CREA enfrentam desafios na comunicação com clientes e na falta de um marketplace especializado. Implementar utilidades adicionais em uma plataforma dedicada pode trazer vantagens práticas.

## Marketplace Especializado

1. **Facilitação na Contratação de Serviços:** Conexão direta entre contratantes e profissionais, simplificando a busca por especialistas qualificados.

2. **Aumento da Visibilidade Profissional:** Destaque das habilidades e experiências dos profissionais, atraindo mais oportunidades de trabalho.

3. **Transparência e Segurança:** Sistemas de avaliação para construir reputações baseadas em feedback de clientes.

## Ferramenta de Gestão Integrada

1. **Eficiência Operacional:** Simplificação de práticas como cálculos fiscais, gestão de projetos e consultoria técnica em um ambiente digital único.

2. **Agilidade na Tomada de Decisões:** Acesso imediato a informações cruciais para decisões rápidas e fundamentadas.

3. **Suporte Estratégico:** Foco em estratégias de crescimento e desenvolvimento de negócios com gestão simplificada de projetos.

Essas utilidades extras promovem eficiência e segurança nas transações técnicas, fortalecendo a posição dos profissionais do CREA no mercado.

---

Licença MIT
