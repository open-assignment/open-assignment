import styled from "styled-components";
import React from "react";

export const OverlayBlockExtension = styled.div`
  position: absolute;
  display: none;
  width: 100%;
  height: 100%;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--black-a1);
  z-index: 2;
  cursor: pointer;
  pointer-events: none;
`;

export const BlockExtensionWrapper = styled.div<{ $selected: boolean }>`
  position: relative;

  &:hover {
    cursor: pointer;
  }
  & ${OverlayBlockExtension} {
    display: ${(props) => (props.$selected ? "block" : "none")};
  }
`;

export type ExtensionWrapperProps = {
  children: React.ReactNode;
  selected: boolean;
};

export const ExtensionWrapper = ({
  children,
  selected,
}: ExtensionWrapperProps) => {
  return (
    <BlockExtensionWrapper $selected={selected}>
      <OverlayBlockExtension />
      {children}
    </BlockExtensionWrapper>
  );
};
