'use client';
import { Inter } from 'next/font/google';
import { AppRouterCacheProvider } from '@mui/material-nextjs/v15-appRouter';
import { ThemeProvider } from '@mui/material/styles';

import './globals.css';
import theme from './theme';
import { Roboto } from 'next/font/google';
import { CssBaseline } from '@mui/material';

const roboto = Roboto({
    weight: ['300', '400', '500', '700'],
    subsets: ['latin'],
    display: 'swap',
    variable: '--font-roboto',
});
const inter = Inter({ subsets: ['latin'] });

export default function RootLayout(props: { children: React.ReactNode }) {
    return (
        <html lang="en" className={roboto.variable}>
            <body className={`${inter.className}`}>
                <AppRouterCacheProvider>
                    <ThemeProvider theme={theme}><CssBaseline />{props.children}</ThemeProvider>
                </AppRouterCacheProvider>
            </body>
        </html>
    );
}