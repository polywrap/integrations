import React from "react";
import { Box, Container, Link, Theme, Typography, useTheme } from "@mui/material";
import { useColorMode } from "@docusaurus/theme-common";
import { makeStyles } from "@mui/styles";

const useStyles = makeStyles((theme: Theme) => ({
  link: {
    color: `${theme.palette.primary.dark} !important`,
    fontWeight: 800,
  },
}));

export default function AdditionalLinks() {
  const theme = useTheme();
  const classes = useStyles(theme);
  const { colorMode } = useColorMode();
  return (
    <Box mt={6} mb={6} position="relative" zIndex={0}>
    <Container maxWidth="lg">
      <Box
        sx={{
          background: `rgba(0,0,0,${colorMode === "dark" ? 0.08 : 0.02})`,
          border: `1px solid ${colorMode === "dark" ? theme.palette.primary.main : "#228eb9"}30`,
          borderRadius: theme.spacing(4),
          display: "block",
          overflow: "hidden",
          padding: theme.spacing(4),
          transform: "translateY(0)",
          transition: "all 0.25s ease-in-out",
          backdropFilter: "blur(16px)",
        }}
      >
          <Box display="flex">
            <Box 
              sx={{
                color: "var(--ifm-heading-color)",
                height: 48,
                mr: 4,
                transform: "translateY(4px)",
                width: 48,
              }}
            >
              <svg width="100%" height="auto" viewBox="0 0 48 48" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                <path d="M19.776 20.136C18.408 20.136 17.328 21.336 17.328 22.8C17.328 24.264 18.432 25.464 19.776 25.464C21.144 25.464 22.224 24.264 22.224 22.8C22.248 21.336 21.144 20.136 19.776 20.136ZM28.536 20.136C27.168 20.136 26.088 21.336 26.088 22.8C26.088 24.264 27.192 25.464 28.536 25.464C29.904 25.464 30.984 24.264 30.984 22.8C30.984 21.336 29.904 20.136 28.536 20.136Z"/>
                <path d="M40.2 0H8.04C5.328 0 3.12 2.208 3.12 4.944V37.392C3.12 40.128 5.328 42.336 8.04 42.336H35.256L33.984 37.896L37.056 40.752L39.96 43.44L45.12 48V4.944C45.12 2.208 42.912 0 40.2 0ZM30.936 31.344C30.936 31.344 30.072 30.312 29.352 29.4C32.496 28.512 33.696 26.544 33.696 26.544C32.712 27.192 31.776 27.648 30.936 27.96C29.736 28.464 28.584 28.8 27.456 28.992C25.152 29.424 23.04 29.304 21.24 28.968C19.872 28.704 18.696 28.32 17.712 27.936C17.16 27.72 16.56 27.456 15.96 27.12C15.888 27.072 15.816 27.048 15.744 27C15.696 26.976 15.672 26.952 15.648 26.928C15.216 26.688 14.976 26.52 14.976 26.52C14.976 26.52 16.128 28.44 19.176 29.352C18.456 30.264 17.568 31.344 17.568 31.344C12.264 31.176 10.248 27.696 10.248 27.696C10.248 19.968 13.704 13.704 13.704 13.704C17.16 11.112 20.448 11.184 20.448 11.184L20.688 11.472C16.368 12.72 14.376 14.616 14.376 14.616C14.376 14.616 14.904 14.328 15.792 13.92C18.36 12.792 20.4 12.48 21.24 12.408C21.384 12.384 21.504 12.36 21.648 12.36C23.112 12.168 24.768 12.12 26.496 12.312C28.776 12.576 31.224 13.248 33.72 14.616C33.72 14.616 31.824 12.816 27.744 11.568L28.08 11.184C28.08 11.184 31.368 11.112 34.824 13.704C34.824 13.704 38.28 19.968 38.28 27.696C38.28 27.696 36.24 31.176 30.936 31.344Z"/>
              </svg>
            </Box>
            <Box>
              <Typography variant="h5" component="h3" fontWeight="800">
                Join us on Discord!
              </Typography>
              <Box mt={1} color={"var(--ifm-heading-color)"}>
                <Typography variant="body1">
                  Join our growing community and get support in real-time.
                </Typography>
              </Box>
              <Box mt={1}>
                <Link href="https://github.com/polywrap/mini-hacks" target="_blank" rel="noredirect" underline="hover" className={classes.link}>
                  Join now &#8250;
                </Link>
              </Box>
            </Box>
          </Box>
      </Box>
    </Container>
    </Box>
  );
} 