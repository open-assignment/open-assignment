import { useEffect, useState } from "react";
import styled from "styled-components";
import { Divider, Input } from "antd";
import { t } from "@lingui/macro";
import { PanelGroup, Panel, PanelResizeHandle } from "react-resizable-panels";

import { DocumentActionPermission } from "graphql/types";
import usePermission from "hook/UsePermission";
import Editor from "../Editor";
import { IPage } from "context/PageStore";
import { useDebounce } from "ahooks";
import useUpdatePage from "hook/UseUpdatePage";
import usePageContentStore from "context/PageContentStore";

export type ContentPageProps = {
  page: IPage;
};

const ContentPage = ({ page }: ContentPageProps) => {
  const allow = usePermission();
  const pageContents = usePageContentStore((state) =>
    state.pageContents.filter((content) => content.pageId === page.id),
  ).sort((a, b) => a.index - b.index);
  const [title, setTitle] = useState(page.title);
  const debouncedTitle = useDebounce(title, { wait: 500 });
  const { upsert } = useUpdatePage(page.id);

  useEffect(() => {
    upsert({
      title: debouncedTitle,
    });
  }, [debouncedTitle]);

  return (
    <div>
      <div style={{ padding: 20 }}>
        <PageTitle
          autoSize
          variant="borderless"
          maxLength={255}
          placeholder={t`Untitled`}
          value={title}
          onChange={(e) => setTitle(e.currentTarget.value)}
          readOnly={!allow(DocumentActionPermission.EDIT_DOCUMENT)}
        />
      </div>
      <Divider style={{ margin: 0 }} />
      <div>
        <PanelGroup direction="horizontal">
          {pageContents.map((pageContent, index) => (
            <>
              <Panel key={pageContent.id} minSize={30}>
                <Editor pageContent={pageContent} />
              </Panel>
              {index !== pageContents.length - 1 && <PanelResizeHandle />}
            </>
          ))}
        </PanelGroup>
      </div>
    </div>
  );
};

export const PageTitle = styled(Input.TextArea)`
  &&& {
    font-size: 20px;
    font-weight: 700;
    padding-left: 0;
    overflow: hidden;
    line-height: normal;

    &:focus {
      outline: none !important;
      box-shadow: none !important;
      border-color: transparent !important;
    }
  }
`;

export default ContentPage;