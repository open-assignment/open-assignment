import { useEffect, useState } from "react";
import styled, { useTheme } from "styled-components";
import { Divider, Input } from "antd";
import { t } from "@lingui/macro";
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";

import { DocumentActionPermission } from "graphql/types";
import usePermission from "hook/UsePermission";
import Editor from "components/Editor";
import { IPage } from "store/PageStore";
import { useDebounce } from "ahooks";
import useUpdatePage from "hook/UseUpdatePage";
import usePageContentStore from "store/PageContentStore";
import useUIStore, { RightSideBarOptions, UIConfig } from "store/UIStore";

const getBodyWidth = (uiConfig: UIConfig) => {
  let bodyWidth = "100vw - 535px";
  // If left sidebar is not visible -> the side should be increase 256px
  if (!uiConfig.leftSidebarVisible) bodyWidth += " + 256px";
  // If right sidebar is not visible -> the side should be increase 256px
  if (uiConfig.rightSideBarVisible !== RightSideBarOptions.None)
    bodyWidth += " + 256px";

  return `calc(${bodyWidth})`;
};

export type ContentPageProps = {
  page: IPage;
};

const ContentPage = ({ page }: ContentPageProps) => {
  const theme = useTheme();
  const allow = usePermission();
  const uiConfig = useUIStore((state) => state.config);
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
      <div style={{ width: getBodyWidth(uiConfig) }}>
        <PanelGroup direction="horizontal">
          {pageContents.map((pageContent, index) => (
            <>
              <Panel key={pageContent.id} minSize={30}>
                <Editor
                  readOnly={!allow(DocumentActionPermission.EDIT_DOCUMENT)}
                  pageContent={pageContent}
                />
              </Panel>
              {index !== pageContents.length - 1 && (
                <PanelResizeHandle
                  style={{
                    width: "1px",
                    backgroundColor: theme.colors.gray[4],
                  }}
                />
              )}
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
