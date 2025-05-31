'use client';
import { createTheme } from '@mui/material/styles';

export default createTheme({
    components: {
        MuiStack: {
            defaultProps: {
                useFlexGap: true,
            },
        },
    },
    typography: {
        fontFamily: 'var(--font-roboto)',
    },
    cssVariables: true,
});