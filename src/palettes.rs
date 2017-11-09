/**************************************************************************
 * Cuddly Potato - An image generator with a REST API
 * Copyright (C) 2017  Jan Solanti
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 *************************************************************************/

pub const DEFAULT: &[(u8,u8,u8)] = &[
    (0x00, 0x00, 0x30),
    (0x00, 0x00, 0x55),
    (0x00, 0x00, 0x77),
    (0x00, 0x00, 0xBB),
    (0x30, 0x3F, 0xFF),
    (0xFF, 0xFF, 0x00),
    (0xFF, 0x7F, 0x00),
    (0x7F, 0x00, 0x00),
    (0x5C, 0x00, 0x00),
    (0x30, 0x00, 0x00),
    (0x00, 0x00, 0x00),
];
