import { gql } from "@apollo/client";

export const SOFT_DELETE_DOCUMENT = gql`
  mutation SoftDeleteDocument($documentId: UUID!, $includeChildren: Boolean!) {
    documentSoftDelete(
      documentId: $documentId
      includeChildren: $includeChildren
    )
  }
`;

export const ADD_DOCUMENT_STANDALONE = gql`
  mutation AddDocumentStandalone(
    $data: NewDocument!
    $spaceId: Int
    $isAssignment: Boolean!
  ) {
    documentCreate(
      data: $data
      spaceId: $spaceId
      isAssignment: $isAssignment
    ) {
      id
      title
      createdAt
      parentId
      index
      documentType
      deletedAt
      iconType
      iconValue
      assignment {
        id
      }
      submission {
        id
      }
      isPrivate
      isDefaultFolderPrivate
    }
  }
`;

export const UPDATE_DOCUMENT_POSITIONS = gql`
  mutation UpdateDocumentPositions($items: [UpdatePositionData!]!) {
    documentUpdatePositions(items: $items)
  }
`;

export const ADD_OR_UPDATE_PAGE = gql`
  mutation AddOrUpdatePage($page: PageInput!, $isSinglePage: Boolean) {
    documentAddOrUpdatePage(page: $page, isSinglePage: $isSinglePage) {
      id
      documentId
      index
      title
      layout
      pageContents {
        id
        pageId
        index
        body
        updatedAt
        createdAt
      }
    }
  }
`;

export const REMOVE_PAGE = gql`
  mutation RemovePage($pageId: UUID!) {
    documentRemovePage(pageId: $pageId)
  }
`;

export const ADD_ORG_UPDATE_PAGE_CONTENT = gql`
  mutation AddOrUpdatePageContent($pageContent: PageContentInput!) {
    documentAddOrUpdatePageContent(pageContent: $pageContent) {
      id
      pageId
      index
      body
      updatedAt
      createdAt
    }
  }
`;

export const ASSIGN_TO_STUDENTS = gql`
  mutation AssignDocument($documentId: UUID!, $emails: [String!]!) {
    documentAssign(documentId: $documentId, emails: $emails)
  }
`;

export const REMOVE_ASSIGNEE = gql`
  mutation RemoveDocumentAssignee($documentId: UUID!, $userId: Int!) {
    documentRemoveAssignee(documentId: $documentId, userId: $userId)
  }
`;

export const UPSERT_WRITING_BLOCK = gql`
  mutation UpsertWritingBlock(
    $pageContentId: UUID!
    $writingBlock: WritingBlockInput!
  ) {
    documentUpsertWritingBlock(
      pageContentId: $pageContentId
      writingBlock: $writingBlock
    ) {
      id
      pageContentId
      creatorId
      content
      updatedAt
      createdAt
    }
  }
`;

export const CLONE_WRITING_BLOCK = gql`
  mutation CloneWritingBlock(
    $writingBlockId: UUID!
    $newWritingBlockId: UUID!
    $newPageContentId: UUID!
  ) {
    documentCloneWritingBlock(
      writingBlockId: $writingBlockId
      newWritingBlockId: $newWritingBlockId
      newPageContentId: $newPageContentId
    ) {
      id
      pageContentId
      creatorId
      content
      updatedAt
      createdAt
    }
  }
`;
